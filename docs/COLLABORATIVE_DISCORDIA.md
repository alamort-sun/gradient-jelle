# Collaborative Discordia — expressive dialogue and design exploration

Implemented 2026-09-25 in `src/dialogue.rs`, with a runnable local chat in `examples/discordia_chat.rs`.

This extension makes Jelle able to carry conversational context into a real text generator. It is prompt conditioning with explicit feedback, not a new trained JEPA, a transfer of hosted model weights, or a measured creativity improvement. The first connected generator is an already installed Ollama model. Other backends implement `TextGenerator`.

## Use

Double-click `Start Discordia.command` in this repository, or run:

```sh
cargo run --offline --example discordia_chat -- qwen3.6:35b-a3b /tmp/discordia-session.json
```

For the command-line invocation, start Ollama first. Use an exact installed model name from `ollama list`. The entry point refuses an unlisted model rather than downloading it. The launcher uses the installed `qwen3.6:35b-a3b` by default; pass another installed model as its first argument. A stronger model can be selected without changing the dialogue layer.

Ordinary lines are sent as conversation. Controls:

| Command | Effect |
|---|---|
| `/play 0.8` | More lively language and rhythm; 0 requests plain language |
| `/design on` | Propose three distinct design directions, palettes, deliberate risks, and experiments |
| `/design off` | Return to ordinary conversation |
| `/bold 0.95` | Encourage departures from conventional composition |
| `/colour 0.95` | Encourage surprising colour relationships |
| `/remember We are making a silk-thread spider pet` | Save a user-specified context note |
| `/prefer I liked the rhythm and the surprising connection` | Mark the previous exchange as a preferred example |
| `/forget` | Remove history, notes, and preferred examples from this session file on the next successful save |
| `/quit` | Exit |

Design mode is off initially so a greeting remains a greeting. Its default boldness and colour freedom are 0.85 and 0.9; the example count is configurable in the library from 1–4. These are expressive instructions, not model temperature, calibrated probabilities, or coordinates that certify beauty.

## What develops over time

The last eight exchanges, up to 32 explicit notes, and eight explicitly preferred examples survive restart in a local JSON file. Preference examples condition later replies. Ordinary outputs are never silently converted into preferred examples or weight updates. The voice guidance encourages useful disagreement, apt metaphors and varied rhythm, while leaving room for plain, precise technical answers.

Generation commits history only on success. Invalid geometry, invalid settings, empty replies, oversized replies, and failed backends do not alter history. Session files are validated on load. The CLI saves via a temporary file and rename, and refuses simultaneous access using a sidecar lock. A hard crash can leave the lock; remove it only after confirming no process uses that session. Forgetting does not erase filesystem backups or provider logs. Use separate session files for separate projects.

## Architecture boundary

`Session::reply` validates and preserves the entire `JelleState`, including both magnetic fields, then generates text from semantic context held separately. It does not use coherence as a factual-confidence score. Replies explicitly carry `factual_accuracy_verified: false`.

This is the conversational generation lane inside Jelle’s crate. It does **not** route generated prose through `Jelle::step` and pretend a codec receipt verifies its meaning. Existing JEPA/MoE classification gates remain intact; learned expert selection and validated semantic evaluation are still future integration work. Text generation can explore speculative designs without manufacturing confidence in factual claims.

The JSON prompt boundary is organizational, not a proven prompt-injection defense. Local model quality still matters. A capable host model using these preferences can be more expressive than the older 0.5B experiment, but this change alone cannot establish improved reasoning or reliable instruction adherence.

## Verification

Six new integration tests cover restart continuity, explicit preference selection, failure rollback, invalid geometry, preservation of both magnetic poles, design conditioning, invalid settings, forgetting, and older session compatibility. Existing gates also pass. Both Mistral and Qwen were exercised locally. Mistral returned ideas but missed the length and palette instructions. After disabling terminal word wrapping, Qwen returned clean text with three distinct compositions, hex colours, risks, and experiments (about 170 words). Some experiments assumed a physical product, so this is not a pass on software-domain fidelity or a blind quality comparison. Initial Qwen loading plus generation took about 92 seconds on this device. No model was downloaded. See `creative-distillation/verification/dialogue-design-smoke.txt` for the raw Qwen reply.

Next meaningful evaluation: collect preferred and rejected design pairs from Evie, compare baseline and conditioned outputs blind across held-out briefs, and score distinctness, expressive fit, functional usefulness, and legibility separately. Keep a held-out set before any future preference training.
