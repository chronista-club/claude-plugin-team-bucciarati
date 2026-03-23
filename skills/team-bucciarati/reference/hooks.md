# Hooks（On Demand）

エージェントが起動されたら、以下のフックをセッションに一時的に登録すること。
エージェント終了時にフックは解除される。

## Sticky Fingers — PreToolUse Bash ガード

**force-push および main/master への直接 push をブロックする。**

判定ロジック:
- Bash ツールの `command` 引数に `git push --force` または `git push -f` が含まれていたら **ブロック**
  - メッセージ: 「force-push は禁止されています。通常の push を使ってください。」
- Bash ツールの `command` 引数に `git push (origin )main` または `git push (origin )master` が含まれていたら **ブロック**
  - メッセージ: 「main/master への直接 push は禁止されています。PR 経由でマージしてください。」

```bash
# フック実装（PreToolUse, matcher: Bash）
INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // ""')
if echo "$COMMAND" | grep -qE 'git\s+push.*--force|git\s+push.*-f\b'; then
  echo "BLOCK: force-push は禁止されています。通常の push を使ってください。"
  exit 2
fi
if echo "$COMMAND" | grep -qE 'git\s+push\s+(origin\s+)?(main|master)\b'; then
  echo "BLOCK: main/master への直接 push は禁止されています。PR 経由でマージしてください。"
  exit 2
fi
```

## Gold Experience — PreToolUse Bash ガード

**破壊的コマンドの実行をブロックする。**

判定ロジック:
- Bash ツールの `command` 引数に以下のパターンが含まれていたら **ブロック**:
  - `rm -rf /` — ルートファイルシステムの削除
  - `DROP TABLE` / `DROP DATABASE` — データベースの破壊
  - `docker system prune` — Docker リソースの一括削除
  - `kubectl delete namespace` — Kubernetes 名前空間の削除
- メッセージ: 「破壊的コマンドが検出されました。本当に実行する場合はユーザーに確認してください。」

```bash
# フック実装（PreToolUse, matcher: Bash）
INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // ""')
if echo "$COMMAND" | grep -qE 'rm\s+-rf\s+/\s*$|DROP\s+(TABLE|DATABASE)|docker\s+system\s+prune|kubectl\s+delete\s+namespace'; then
  echo "BLOCK: 破壊的コマンドが検出されました。本当に実行する場合はユーザーに確認してください。"
  exit 2
fi
```
