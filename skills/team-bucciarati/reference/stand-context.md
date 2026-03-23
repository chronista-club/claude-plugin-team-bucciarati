# StandContext 仕様

## 構造

パイプライン内で各スタンド間に引き継がれるコンテキスト。
各スタンドの結果を次のスタンドに渡す際、以下の構造化フォーマットを使用する:

```
## StandContext

### Source
stand: <前のスタンド名>
status: <DONE / BLOCKED / ERROR>

### Artifacts
branch: <ブランチ名>
pr_number: <PR 番号>
pr_url: <PR URL>
deploy_url: <デプロイ URL>
ci_status: <PASS / FAIL>

### Issue
type: <linear / github>
id: <Linear ID or Issue 番号>
title: <Issue タイトル>

### Notes
<前のスタンドからの引き継ぎメモ>
```

**全てのフィールドはオプショナル。** 該当するものだけ埋める。
各スタンドの prompt にこの StandContext を含めることで、情報の欠落を防ぐ。

## Issue コンテキスト

### Linear Issues（デフォルト）

ユーザーが Linear Issue ID を指定した場合（例: `VP-9 をやって`）:

- **Issue 取得**: `get_issue(id: "VP-9")` で内容を把握
- **ブランチ命名**: `gitBranchName` を使用（`mako/{team-key}-XX-...` 形式）
- **ステータス更新**: 実装開始時に `save_issue(state: "In Progress")`
- **完了時**: `save_issue(state: "Done")`
- **Release リンク**: リリース後に `save_issue(links: [{url: "リリースURL", title: "Release vX.Y.Z"}])`
- **PR リンク**: PR body に `Closes VP-9` を含める（Linear の GitHub 連携で自動クローズ）
- Linear MCP が使えない場合はスキップ（パイプラインは止めない）

Issue コンテキストは StandContext に含めて各スタンドに引き継ぐ。

### GitHub Issues（レガシー）

GitHub Issues が有効なリポジトリでのみ使用:

- **ブランチ名**: `feat/<Issue番号>-<slug>`
- **PR リンク**: `Closes #N` を PR body に自動挿入
- **完了時**: マージ時の `Closes #N` で自動クローズ
