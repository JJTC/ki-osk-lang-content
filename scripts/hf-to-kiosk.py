#!/usr/bin/env python3
"""Convert a HuggingFace decoder transformer into a ki-osk neural bundle.

Reads a directory containing the canonical HF model files (`config.json`,
`tokenizer.json`, `model.safetensors`), validates that the architecture
falls within ki-osk's supported subset (vanilla decoder, GELU MLP,
learned position embeddings — see `BRING_YOUR_OWN_WEIGHTS.md`), and
writes a ki-osk bundle to a target directory.

Usage:
    ./scripts/hf-to-kiosk.py \\
        --hf-dir /path/to/hf/model \\
        --out /usr/share/ki-osk/models/multilingual

The script does NOT download anything. Use the HuggingFace `huggingface-cli`
or `transformers` library to fetch a model into a local dir first, then
point this script at it.

Validation rejects models that use:
  - Rotary position embeddings (RoPE)
  - SwiGLU / GeGLU / gated MLPs
  - Grouped-query attention (GQA), multi-query attention (MQA)
  - Mixture of experts
  - Tied input/output embeddings (we always train with a separate head)

If your HF model uses any of those, distill it into the supported
subset first; that's the path documented in BRING_YOUR_OWN_WEIGHTS.md.
"""

from __future__ import annotations

import argparse
import json
import shutil
import sys
from pathlib import Path


SUPPORTED_ACT = {"gelu", "gelu_new", "gelu_pytorch_tanh"}
"""Activation functions that match ki-osk's `Block::forward` (which uses
candle's `Tensor::gelu`). Anything outside this set means the runtime
will produce numerically different outputs from training."""


def fail(msg: str) -> None:
    print(f"ERROR: {msg}", file=sys.stderr)
    sys.exit(2)


def validate_hf_config(hf_cfg: dict) -> None:
    """Reject HF configs that use features outside our v1 envelope."""
    arch = hf_cfg.get("architectures", ["?"])[0].lower()
    print(f"hf architecture: {arch}")

    # RoPE detection: most HF configs expose `rope_theta` /
    # `rope_scaling` for rotary models. Some older Llama-style
    # configs only have `rotary_pct`.
    if "rope_theta" in hf_cfg or "rope_scaling" in hf_cfg:
        fail("model uses rotary position embeddings (RoPE) — distill to "
             "learned-position first; see BRING_YOUR_OWN_WEIGHTS.md")
    if hf_cfg.get("rotary_pct"):
        fail("model uses rotary position embeddings (rotary_pct set)")

    # Gated MLP detection: SwiGLU/GeGLU configs typically have an
    # `intermediate_size` 8/3× the hidden, plus an `up_proj` weight
    # alongside `gate_proj`. Easier signal: the activation name.
    act = (hf_cfg.get("hidden_act") or hf_cfg.get("activation_function") or "").lower()
    print(f"hf hidden_act: {act}")
    if act not in SUPPORTED_ACT:
        fail(f"activation {act!r} not in supported set {sorted(SUPPORTED_ACT)} — "
             "model likely uses SwiGLU / GeGLU; distill to vanilla GELU first")

    # GQA / MQA: HF configs distinguish via `num_key_value_heads`
    # vs `num_attention_heads`. They're equal for vanilla MHA.
    n_heads = hf_cfg.get("num_attention_heads") or hf_cfg.get("n_head")
    n_kv_heads = hf_cfg.get("num_key_value_heads", n_heads)
    if n_heads is None:
        fail("hf config missing num_attention_heads / n_head")
    if n_kv_heads != n_heads:
        fail(f"GQA/MQA detected (n_heads={n_heads}, n_kv_heads={n_kv_heads}) — "
             "ki-osk's Block uses standard MHA only")

    # Mixture of experts.
    if hf_cfg.get("num_experts") and hf_cfg["num_experts"] > 1:
        fail(f"MoE model (num_experts={hf_cfg['num_experts']}) — not supported")

    # Tied embeddings.
    if hf_cfg.get("tie_word_embeddings"):
        fail("tie_word_embeddings=True — ki-osk's CharLm has a separate `head` "
             "Linear; untied weights are required")


def emit_kiosk_config(hf_cfg: dict) -> dict:
    """Translate a validated HF config into ki-osk's `ModelConfig` shape."""
    # Map HF field names to ki-osk's flatter shape. HF varies between
    # GPT-2 (`n_layer` / `n_head` / `n_embd`) and newer (`num_*` /
    # `hidden_size`) naming; handle both.
    n_layer = hf_cfg.get("n_layer") or hf_cfg.get("num_hidden_layers")
    n_head  = hf_cfg.get("n_head")  or hf_cfg.get("num_attention_heads")
    n_embd  = hf_cfg.get("n_embd")  or hf_cfg.get("hidden_size")
    block_size = (
        hf_cfg.get("n_positions") or hf_cfg.get("max_position_embeddings")
        or hf_cfg.get("max_sequence_length")
    )
    vocab_size = hf_cfg.get("vocab_size")

    for name, val in [
        ("n_layer", n_layer), ("n_head", n_head), ("n_embd", n_embd),
        ("block_size", block_size), ("vocab_size", vocab_size),
    ]:
        if val is None:
            fail(f"hf config missing {name} (and the equivalent alternate names)")

    return {
        "n_layer": int(n_layer),
        "n_head": int(n_head),
        "n_embd": int(n_embd),
        "block_size": int(block_size),
        "vocab_size": int(vocab_size),
        "tokenizer": "bpe",
    }


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--hf-dir", type=Path, required=True,
                    help="HuggingFace model dir (contains config.json + tokenizer.json + model.safetensors)")
    ap.add_argument("--out", type=Path, required=True,
                    help="Target ki-osk bundle dir (e.g. /usr/share/ki-osk/models/multilingual)")
    args = ap.parse_args()

    hf_cfg_path = args.hf_dir / "config.json"
    tok_path = args.hf_dir / "tokenizer.json"
    st_path = args.hf_dir / "model.safetensors"
    for p in (hf_cfg_path, tok_path, st_path):
        if not p.exists():
            fail(f"missing required file: {p}")

    hf_cfg = json.loads(hf_cfg_path.read_text())
    validate_hf_config(hf_cfg)
    kiosk_cfg = emit_kiosk_config(hf_cfg)

    args.out.mkdir(parents=True, exist_ok=True)
    (args.out / "config.json").write_text(json.dumps(kiosk_cfg, indent=2) + "\n")
    shutil.copy2(tok_path, args.out / "tokenizer.json")
    shutil.copy2(st_path, args.out / "model.safetensors")

    print(f"wrote ki-osk bundle to {args.out}")
    print(f"  config.json     ({kiosk_cfg})")
    print(f"  tokenizer.json  ({tok_path.stat().st_size} B)")
    print(f"  model.safetensors ({st_path.stat().st_size} B)")
    print()
    print("Next: validate via")
    print(f"  cargo run --release --features neural --bin validate-neural -- {args.out}")


if __name__ == "__main__":
    main()
