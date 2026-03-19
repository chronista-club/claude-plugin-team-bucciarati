#!/usr/bin/env bash
# プロジェクトの CI ツールチェーンを検出して JSON で返す
# Usage: ./scripts/detect-ci.sh [project-root]

set -euo pipefail

ROOT="${1:-.}"
cd "$ROOT"

if [ -f "mise.toml" ] || [ -f ".mise.toml" ]; then
  RUNNER="mise"
  CMDS=()
  # mise tasks から利用可能なタスクを検出
  for task in typecheck lint build test; do
    if mise task ls 2>/dev/null | grep -q "^$task"; then
      CMDS+=("mise run $task")
    fi
  done
  if [ ${#CMDS[@]} -eq 0 ]; then
    CMDS=("mise run typecheck" "mise run lint" "mise run build" "mise run test")
  fi
elif [ -f "Cargo.toml" ]; then
  RUNNER="cargo"
  CMDS=("cargo clippy -- -D warnings" "cargo build --release" "cargo test")
elif [ -f "package.json" ]; then
  RUNNER="node"
  # package manager 検出
  if [ -f "bun.lockb" ] || [ -f "bun.lock" ]; then
    PM="bun"
  elif [ -f "pnpm-lock.yaml" ]; then
    PM="pnpm"
  elif [ -f "yarn.lock" ]; then
    PM="yarn"
  else
    PM="npm"
  fi
  CMDS=()
  for script in typecheck lint build test; do
    if grep -q "\"$script\"" package.json 2>/dev/null; then
      CMDS+=("$PM run $script")
    fi
  done
  if [ ${#CMDS[@]} -eq 0 ]; then
    CMDS+=("$PM test")
  fi
  RUNNER="$PM"
elif [ -f "go.mod" ]; then
  RUNNER="go"
  CMDS=("go vet ./..." "go build ./..." "go test ./...")
else
  echo '{"runner":"unknown","commands":[]}'
  exit 0
fi

# JSON 出力
printf '{"runner":"%s","commands":[' "$RUNNER"
for i in "${!CMDS[@]}"; do
  [ "$i" -gt 0 ] && printf ','
  printf '"%s"' "${CMDS[$i]}"
done
printf ']}\n'
