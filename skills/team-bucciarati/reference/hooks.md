# Hooks（On Demand）

エージェントが起動されたら、以下のフックをセッションに一時的に登録すること。
エージェント終了時にフックは解除される。

## 全スタンド共通 — コミットライン・ガード（PreToolUse Bash）

**team-b はコミットラインを越えない。** commit / push / PR / merge / デプロイ系コマンドをブロックする。

判定ロジック:
- Bash ツールの `command` 引数に `git commit`, `git push`, `git merge`, `gh pr` が含まれていたら **ブロック**
  - メッセージ: 「team-b の終点は『コミット可能な diff』まで。コミット以降はメインセッションで行ってください。」

```bash
# フック実装（PreToolUse, matcher: Bash）
INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // ""')
if echo "$COMMAND" | grep -qE 'git\s+(commit|push|merge)\b|gh\s+pr\b'; then
  echo "BLOCK: team-b の終点は『コミット可能な diff』まで。コミット以降はメインセッションで行ってください。"
  exit 2
fi
```

> `git stash` / `git checkout` / `git restore` はブロックしない — スタンドが作業ステップの巻き戻しに使う。

## 全スタンド共通 — 破壊コマンド・ガード（PreToolUse Bash）

**破壊的コマンドの実行をブロックする。**

判定ロジック:
- Bash ツールの `command` 引数に以下のパターンが含まれていたら **ブロック**:
  - `rm -rf /` — ルートファイルシステムの削除
  - `DROP TABLE` / `DROP DATABASE` — データベースの破壊
  - `git reset --hard` + `git clean -f` の組み合わせ — working tree の全消去
- メッセージ: 「破壊的コマンドが検出されました。本当に実行する場合はユーザーに確認してください。」

```bash
# フック実装（PreToolUse, matcher: Bash）
INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // ""')
if echo "$COMMAND" | grep -qE 'rm\s+-rf\s+/\s*$|DROP\s+(TABLE|DATABASE)|git\s+reset\s+--hard.*&&.*git\s+clean|git\s+clean\s+-f.*&&.*git\s+reset\s+--hard'; then
  echo "BLOCK: 破壊的コマンドが検出されました。本当に実行する場合はユーザーに確認してください。"
  exit 2
fi
```
