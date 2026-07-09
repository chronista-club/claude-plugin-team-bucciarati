#!/usr/bin/env bash
# check-teamdef.sh — team.kdl (SSOT) とドキュメント群のドリフト検証
#
# team.kdl が正とする構造化ファクト（ロスター・モデル配分・パイプライン・
# コミットライン境界・version 同期）を全ドキュメントと突き合わせる。
# ロスターやパイプラインを変更したら、team.kdl を更新してこれを実行すること。
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
exec cargo run --quiet --manifest-path "$ROOT/mcp-server/Cargo.toml" --bin teamb_check -- --root "$ROOT"
