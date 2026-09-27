#!/bin/zsh
set -eu
export PATH="/opt/homebrew/bin:$PATH"
cd "$(dirname "$0")"
data_dir="$HOME/Library/Application Support/gradient-jelle"
mkdir -p "$data_dir"
if ! command -v ollama >/dev/null || ! command -v cargo >/dev/null; then
  echo 'Discordia needs the existing Ollama and Rust installations.'
  read '?Press Return to close.'
  exit 1
fi
if ! ollama list >/dev/null 2>&1; then
  OLLAMA_HOST=127.0.0.1:11434 nohup ollama serve > "$data_dir/ollama.log" 2>&1 &
  for attempt in {1..20}; do
    if ollama list >/dev/null 2>&1; then break; fi
    sleep 0.5
  done
fi
rustup run stable cargo run --offline --example discordia_chat -- "${1:-qwen3.6:35b-a3b}" "$data_dir/discordia-session.json"
read '?Press Return to close.'
