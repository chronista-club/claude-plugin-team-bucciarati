#!/usr/bin/env bash
# nightly-release.sh — main に積まれた未リリースコミットを nightly prerelease に棚卸しする
#
# フロー:
#   1. origin/main を fetch し、前回 nightly / stable タグ以降の新コミットを確認（なければ即終了）
#   2. 一時 worktree で品質ゲート（check-teamdef + cargo test + cargo clippy）
#   3. green なら nightly-YYYYMMDD タグ + GitHub prerelease を作成
#
# nightly は plugin.json / marketplace に一切触れない（git スナップショット + prerelease のみ）。
# 安定版 cut は人間の判断で行う。
#
# Usage: nightly-release.sh [--dry-run]
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DRY_RUN=false
[[ "${1:-}" == "--dry-run" ]] && DRY_RUN=true

cd "$ROOT"
git fetch origin main --tags --quiet
HEAD_SHA=$(git rev-parse origin/main)
TODAY=$(date +%Y%m%d)
TAG="nightly-$TODAY"

# --- 1. 新コミット確認 ---
# 基準点: 最新の nightly-* タグ、なければ最新の v* タグ
LAST_TAG=$(git tag --list 'nightly-*' --sort=-creatordate | head -1)
[[ -z "$LAST_TAG" ]] && LAST_TAG=$(git tag --list 'v*' --sort=-creatordate | head -1)
if [[ -z "$LAST_TAG" ]]; then
  echo "基準タグが見つかりません（v* / nightly-* なし）" >&2
  exit 1
fi

LAST_SHA=$(git rev-parse "$LAST_TAG^{commit}")
if [[ "$HEAD_SHA" == "$LAST_SHA" ]]; then
  echo "✔ 積みなし — origin/main は $LAST_TAG から変化なし。今夜は静かだ。"
  exit 0
fi

COMMITS=$(git log --oneline "$LAST_SHA..$HEAD_SHA")
COUNT=$(echo "$COMMITS" | wc -l | tr -d ' ')
echo "📦 $LAST_TAG 以降に $COUNT コミット:"
echo "$COMMITS"

if git rev-parse "$TAG" >/dev/null 2>&1; then
  echo "✔ $TAG は既に存在（本日分は棚卸し済み）"
  exit 0
fi

# --- 2. 品質ゲート（一時 worktree、作業中の tree を汚さない） ---
WT=$(mktemp -d /tmp/teamb-nightly.XXXXXX)
cleanup() { git worktree remove --force "$WT" >/dev/null 2>&1 || true; rm -rf "$WT"; }
trap cleanup EXIT
git worktree add --detach "$WT" "$HEAD_SHA" --quiet

echo "🔍 品質ゲート実行中 (worktree: $WT)"
GATE_LOG=$(mktemp)
gate() {
  local name="$1"; shift
  if "$@" >>"$GATE_LOG" 2>&1; then
    echo "  ✔ $name"
  else
    echo "  ✘ $name FAILED" >&2
    tail -30 "$GATE_LOG" >&2
    echo "🛑 nightly 中止 — main が品質ゲートを通らない。要調査。" >&2
    exit 1
  fi
}
gate "check-teamdef" bash "$WT/scripts/check-teamdef.sh"
gate "cargo test" cargo test --quiet --manifest-path "$WT/mcp-server/Cargo.toml"
gate "cargo clippy" cargo clippy --quiet --manifest-path "$WT/mcp-server/Cargo.toml" --all-targets -- -D warnings

# --- 3. タグ + prerelease ---
if $DRY_RUN; then
  echo "🌙 [dry-run] $TAG ($HEAD_SHA) を作成する（実際には何もしない）"
  exit 0
fi

git tag "$TAG" "$HEAD_SHA"
git push origin "$TAG" --quiet

NOTES=$(printf '## Nightly %s\n\n`%s` 以降の積み荷 (%s コミット):\n\n```\n%s\n```\n\n品質ゲート: check-teamdef ✔ / cargo test ✔ / clippy ✔\n\n> nightly は git スナップショットのみ。plugin.json / marketplace は不変。安定版は手動 cut。' \
  "$TODAY" "$LAST_TAG" "$COUNT" "$COMMITS")
gh release create "$TAG" --prerelease --title "Nightly $TODAY" --notes "$NOTES" >/dev/null
echo "🌙 $TAG 作成完了 — $COUNT コミットを棚卸し"
