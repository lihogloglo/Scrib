#!/usr/bin/env bash
# Context size of each recent Claude Code session in this project, read from the transcripts' last usage record
# (cache read + cache creation + input tokens = what the next turn re-sends). Meta-coordinator use: lanes hand off
# at ~200k (CONTRIBUTING "Working with coding agents"). Usage: bash tools/session_context.sh [count, default 10]
dir="$HOME/.claude/projects/D--Gamedev-Bevyrrowind"
n="${1:-10}"
ls -t "$dir"/*.jsonl 2>/dev/null | head -n "$n" | while read -r f; do
  line=$(tac "$f" | grep -m1 '"usage"')
  sum=0
  for k in input_tokens cache_read_input_tokens cache_creation_input_tokens; do
    v=$(printf '%s' "$line" | grep -oE "\"$k\":[0-9]+" | head -1 | cut -d: -f2)
    sum=$((sum + ${v:-0}))
  done
  name=$(grep -m1 -oE 'orchestrator \\?"[A-Za-z0-9_-]+|_prompts/[A-Za-z0-9_-]+\.md|meta-coordinator' "$f" | head -1 \
    | sed -E 's#orchestrator \\?"##; s#_prompts/##; s#\.md$##')
  printf '%4dk  %s  %s  %s\n' $((sum / 1000)) "$(date -r "$f" +%H:%M)" "$(basename "$f" .jsonl | cut -c1-8)" "${name:-?}"
done
