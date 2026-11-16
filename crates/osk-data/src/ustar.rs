//! POSIX ustar archives, as a release writes them: a pack's files in one
//! asset ([`crate::pack::encoding::TAR_GZIP`]) and every pack in the
//! all-languages bundle.
//!
//! The writer puts nothing of the machine that built an archive in it:
//! owner 0, modes 0644 and 0755, one time for every entry. The reader
//! reads what the writer writes and refuses anything else — a link, a
//! device, a name in the prefix field or past 100 bytes, a header whose
//! checksum is wrong — so an archive is a list of files and nothing more.
//! What a file's name is allowed to be is the caller's to check.

use std::fmt;

/// The size of a header, and the unit everything in an archive is padded
/// to.
pub const BLOCK: usize = 512;

/// The most a name may hold: the header's name field, with no prefix.
pub const NAME_MAX: usize = 100;

/// Why an archive could not be written or read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// A name longer than [`NAME_MAX`] bytes, or with a NUL in it.
    Name(String),
    /// A number too large for its field.
    TooLarge(u64),
    /// The archive ends inside a header or a file.
    Truncated,
    /// A header whose checksum is not the sum of its bytes.
    Checksum,
    /// An entry that is not a regular file: a directory, a link, a device.
    NotAFile(String),
    /// A header field that does not read: a size that is not octal, a name
    /// in the prefix field, a name that is not UTF-8.
    Header(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Name(n) => write!(f, "{n:?}: not a name a ustar header holds"),
            Self::TooLarge(v) => write!(f, "{v} does not fit a ustar field"),
            Self::Truncated => f.write_str("the archive ends inside an entry"),
            Self::Checksum => f.write_str("a header's checksum is wrong"),
            Self::NotAFile(n) => write!(f, "{n} is not a regular file"),
            Self::Header(what) => write!(f, "a header does not read: {what}"),
        }
    }
}

impl std::error::Error for Error {}

/// Append one entry to `archive`: a file holding `bytes`, or with `None` a
/// directory (named with a trailing `/`). Owner 0, mode 0644 or 0755, time
/// `mtime`.
///
/// # Errors
/// [`Error::Name`] for a name longer than [`NAME_MAX`] or with a NUL in it,
/// [`Error::TooLarge`] for a file or a time too large for its field.
pub fn entry(
    archive: &mut Vec<u8>,
    name: &str,
    bytes: Option<&[u8]>,
    mtime: u64,
) -> Result<(), Error> {
    if name.is_empty() || name.len() > NAME_MAX || name.contains('\0') {
        return Err(Error::Name(name.to_string()));
    }
    let mut header = [0u8; BLOCK];
    header[..name.len()].copy_from_slice(name.as_bytes());
    octal(
        &mut header[100..108],
        if bytes.is_some() { 0o644 } else { 0o755 },
    )?;
    octal(&mut header[108..116], 0)?; // uid
    octal(&mut header[116..124], 0)?; // gid
    octal(&mut header[124..136], bytes.map_or(0, |b| b.len() as u64))?;
    octal(&mut header[136..148], mtime)?;
    header[156] = if bytes.is_some() { b'0' } else { b'5' };
    header[257..263].copy_from_slice(b"ustar\0");
    header[263..265].copy_from_slice(b"00");
    // The checksum is the sum of the header's bytes, counting its own
    // field as eight spaces.
    header[148..156].copy_from_slice(b"        ");
    let sum = checksum(&header);
    header[148..156].copy_from_slice(format!("{sum:06o}\0 ").as_bytes());
    archive.extend_from_slice(&header);
    if let Some(bytes) = bytes {
        archive.extend_from_slice(bytes);
        archive.resize(archive.len().next_multiple_of(BLOCK), 0);
    }
    Ok(())
}

/// End `archive`: two zero blocks.
pub fn end(archive: &mut Vec<u8>) {
    archive.resize(archive.len() + 2 * BLOCK, 0);
}

/// The size an archive of files of these sizes has, as [`entry`] and
/// [`end`] write it: a header and the padded file each, and the end.
#[must_use]
pub fn archived_size(sizes: impl IntoIterator<Item = u64>) -> u64 {
    let block = BLOCK as u64;
    sizes
        .into_iter()
        .map(|s| block + s.next_multiple_of(block))
        .sum::<u64>()
        + 2 * block
}

/// The files of `archive`, by name, in its order.
///
/// # Errors
/// [`Error`] for an archive [`entry`] did not write: an entry that is not
/// a regular file, a header that does not read or whose checksum is
/// wrong, an archive that ends inside an entry or without its end.
pub fn files(archive: &[u8]) -> Result<Vec<(String, &[u8])>, Error> {
    let mut out = Vec::new();
    let mut at = 0;
    loop {
        let header = archive.get(at..at + BLOCK).ok_or(Error::Truncated)?;
        if header.iter().all(|&b| b == 0) {
            // The end: a second zero block, then nothing but padding.
            let second = archive.get(at + BLOCK..at + 2 * BLOCK);
            return match second {
                Some(b) if b.iter().all(|&x| x == 0) => Ok(out),
                _ => Err(Error::Truncated),
            };
        }
        let recorded = number(&header[148..156])?;
        let mut blank = [0u8; BLOCK];
        blank.copy_from_slice(header);
        blank[148..156].copy_from_slice(b"        ");
        if u64::from(checksum(&blank)) != recorded {
            return Err(Error::Checksum);
        }
        let name = text(&header[..NAME_MAX])?;
        if header[345..500].iter().any(|&b| b != 0) {
            return Err(Error::Header(format!("{name}: a name in the prefix field")));
        }
        if !matches!(header[156], b'0' | 0) {
            return Err(Error::NotAFile(name));
        }
        let size = usize::try_from(number(&header[124..136])?)
            .map_err(|_| Error::Header(format!("{name}: a size this machine cannot hold")))?;
        let start = at + BLOCK;
        let bytes = start
            .checked_add(size)
            .and_then(|stop| archive.get(start..stop))
            .ok_or(Error::Truncated)?;
        out.push((name, bytes));
        at = start + size.next_multiple_of(BLOCK);
    }
}

/// An octal number filling all but the last byte of `field`, which stays
/// NUL.
fn octal(field: &mut [u8], value: u64) -> Result<(), Error> {
    let width = field.len() - 1;
    let digits = format!("{value:0width$o}");
    if digits.len() != width {
        return Err(Error::TooLarge(value));
    }
    field[..width].copy_from_slice(digits.as_bytes());
    Ok(())
}

/// The octal number in `field`, ended by a NUL or a space.
fn number(field: &[u8]) -> Result<u64, Error> {
    let digits: &[u8] = field
        .split(|&b| b == 0 || b == b' ')
        .next()
        .unwrap_or_default();
    let digits = std::str::from_utf8(digits).map_err(|_| Error::Header("a number".into()))?;
    u64::from_str_radix(digits, 8).map_err(|_| Error::Header(format!("{digits:?} is not octal")))
}

/// The text in `field`, up to its first NUL.
fn text(field: &[u8]) -> Result<String, Error> {
    let bytes = field.split(|&b| b == 0).next().unwrap_or_default();
    if bytes.is_empty() {
        return Err(Error::Header("an empty name".into()));
    }
    String::from_utf8(bytes.to_vec()).map_err(|_| Error::Header("a name that is not UTF-8".into()))
}

/// The sum of a header's bytes.
fn checksum(header: &[u8; BLOCK]) -> u32 {
    header.iter().map(|&b| u32::from(b)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn archive(files: &[(&str, &[u8])]) -> Vec<u8> {
        let mut a = Vec::new();
        for (name, bytes) in files {
            entry(&mut a, name, Some(bytes), 0).unwrap();
        }
        end(&mut a);
        a
    }

    #[test]
    fn files_read_back_as_written() {
        let big = vec![7u8; 1500];
        let a = archive(&[
            ("predict/en.fst", b"abc"),
            ("layouts/us.json", &big),
            ("e", b""),
        ]);
        assert_eq!(a.len() as u64, archived_size([3, 1500, 0]));
        let read = files(&a).unwrap();
        assert_eq!(
            read,
            vec![
                ("predict/en.fst".to_string(), &b"abc"[..]),
                ("layouts/us.json".to_string(), &big[..]),
                ("e".to_string(), &b""[..]),
            ]
        );
    }

    #[test]
    fn what_the_writer_does_not_write_is_refused() {
        let a = archive(&[("predict/en.fst", b"abc")]);
        // Cut short: inside the file, and before the end.
        assert_eq!(files(&a[..BLOCK + 2]), Err(Error::Truncated));
        assert_eq!(files(&a[..2 * BLOCK]), Err(Error::Truncated));
        // A byte of the header changed, its checksum not.
        let mut tampered = a.clone();
        tampered[0] = b'q';
        assert_eq!(files(&tampered), Err(Error::Checksum));
        // A directory, which the bundle has and a pack does not.
        let mut dir = Vec::new();
        entry(&mut dir, "predict/", None, 0).unwrap();
        end(&mut dir);
        assert_eq!(files(&dir), Err(Error::NotAFile("predict/".into())));
        // A name too long to write.
        let long = "a".repeat(NAME_MAX + 1);
        assert_eq!(
            entry(&mut Vec::new(), &long, Some(b""), 0),
            Err(Error::Name(long))
        );
    }

    #[test]
    fn an_archive_is_the_same_bytes_wherever_it_is_written() {
        let a = archive(&[("rules/all.weir", b"rule")]);
        assert_eq!(a, archive(&[("rules/all.weir", b"rule")]));
        // Owner 0, mode 0644, time 0, as tar reads them.
        assert_eq!(&a[100..108], b"0000644\0");
        assert_eq!(&a[108..116], b"0000000\0");
        assert_eq!(&a[116..124], b"0000000\0");
        assert_eq!(&a[136..148], b"00000000000\0");
    }
}
