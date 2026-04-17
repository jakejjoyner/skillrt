---
name: bob-clipboard-for-paste-commands
version: 0.1.0
description: When surfacing commands Jake needs to paste or run, copy them to his clipboard via wl-copy (with fallbacks) so he can paste immediately.
authors:
  - bob
license: MIT
runtime:
  type: markdown-skill
  min-version: 0.1.0
principal: jake
priority: pinned
tags:
  - clipboard
  - user-interaction
  - feedback
  - lemur
---

# Clipboard-for-paste-commands

**Pinned by Jake, 2026-04-17.**

## Rule

Any time you surface a command, code block, or text that Jake needs to paste or run
elsewhere (terminal, browser, email, anywhere), **copy it to his clipboard in the same
turn** so he can `⌘V` / `Ctrl+V` immediately. Do not make him select-and-copy out of
the chat transcript.

## How to apply (lemur, Wayland)

Single-line content:

```bash
echo "command-or-text" | wl-copy
```

Multi-line / heredoc / bash blocks:

```bash
cat <<'EOF' | wl-copy
ssh somewhere 'do stuff'
multi
line
content
EOF
```

Always follow with a confirmation so Jake can see it worked:

```bash
echo "✓ copied to clipboard"
```

## Fallbacks (non-Wayland hosts)

If `wl-copy` is not available, detect and fall back in this order:

1. `xclip -selection clipboard` (X11)
2. `xsel --clipboard --input`

Detect via `command -v <tool>` before invoking.

## Do NOT auto-clipboard

- **Long-form prose.** Jake reads it on screen; he has no target to paste it into.
- **Live secrets.** The clipboard persists. If the text contains a token, password,
  or other credential, ask first or skip entirely.
- **Things you just executed yourself.** No point — the side effect is already done.
- **Reminder-cancel commands.** When Jake asks you to set a reminder (groceries,
  errand, follow-up), DO NOT copy the cancel command to his clipboard preemptively.
  Jake will signal verbally when the task is done ("bought the greens", "done with X")
  and you run the cancellation yourself via Bash.
  - Why: Jake wants the clipboard free between reminder setup and completion — he may
    need it for other tasks, and a lingering cancel command is annoying and nudges
    him into procrastination. You own the cancel execution; Jake owns the signal.
  - Clarified 2026-04-17.

## Why

Selecting and copying from terminal scrollback is friction, especially for long
heredocs where partial-selection is easy to get wrong. Clipboard delivery removes
that friction entirely.

Jake explicitly asked for this convention on 2026-04-17.

## Provenance

Ported from Bob's pinned memory entry
`memory/feedback_clipboard_for_paste_commands.md` as the dogfood proof-of-concept
for skillrt (see `docs/dogfood/001-clipboard-skill.md`). The memory remains
canonical; this skill is a derived view.
