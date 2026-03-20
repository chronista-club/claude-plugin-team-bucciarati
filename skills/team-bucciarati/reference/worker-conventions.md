# Worker 通信規約

Sex Pistols が使用する Worker 間の構造化メッセージと PP 表示規約。

## wire_send メッセージフォーマット

### タスク指示（Lead → Worker）

```json
{
  "type": "task",
  "issue": "NEX-12",
  "title": "ユーザー認証のリファクタリング",
  "mode": "relay",
  "branch": "mako/nex-12-auth-refactor",
  "context": "verify_token を async に変更。check_auth は非推奨化",
  "acceptance": [
    "テストが通る",
    "既存 API に breaking change なし"
  ]
}
```

### 進捗報告（Worker → Lead）

```json
{
  "type": "progress",
  "issue": "NEX-12",
  "status": "in_progress",
  "phase": "実装中",
  "completed": ["auth/handler.rs 読了", "auth/middleware.rs 読了"],
  "current": "handler.rs リファクタ中",
  "remaining": ["テスト作成", "middleware 更新"],
  "diff_summary": "+42 -18 (2 files)"
}
```

### 質問（Worker → Lead）

```json
{
  "type": "question",
  "issue": "NEX-12",
  "question": "check_auth を完全削除していい？呼び出し元が3箇所ある",
  "options": ["完全削除", "deprecated マーク", "wrapper で互換維持"],
  "context": "src/api/routes.rs:42, src/api/admin.rs:18, src/middleware/auth.rs:7"
}
```

### 完了報告（Worker → Lead）

```json
{
  "type": "done",
  "issue": "NEX-12",
  "summary": "verify_token を async 化。check_auth は deprecated マーク",
  "diff_summary": "+87 -34 (5 files)",
  "pr_ready": true
}
```

## PP 表示規約（VP TUI 連携時）

Worker が `vp pane show` で自分の PP ペインに表示する内容。

### autonomous（指揮者）

```markdown
**Task: NEX-12** | 🎵 指揮者 | 🔨 実装中
```

### relay（監督）

```markdown
**Task: NEX-12** | Mode: 🎬 監督
Branch: mako/nex-12-auth-refactor
───────────────────────
**Progress:**
✅ auth/handler.rs 読了
✅ auth/middleware.rs 読了
🔨 handler.rs リファクタ中
⬚ テスト作成
⬚ middleware 更新
───────────────────────
**Design Notes:**
- verify_token を async に変更
- check_auth は deprecated マーク
───────────────────────
**Diff: +42 -18 (2 files)**
```

### pair（ペアプロ）

relay の内容に加えて:

```markdown
───────────────────────
**対話ログ:**
[Lead] check_auth は完全削除でいい？
[Worker] 呼び出し元3箇所。deprecated マークを提案
[Lead] OK、deprecated で

**共有メモ:**
- 認証フロー図を docs/auth-flow.md に追加予定
- v2 API では check_auth を完全削除
```
