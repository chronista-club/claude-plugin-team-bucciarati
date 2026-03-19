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
type: <github / linear>
id: <Issue 番号 or Linear ID>
title: <Issue タイトル>

### Notes
<前のスタンドからの引き継ぎメモ>
```

**全てのフィールドはオプショナル。** 該当するものだけ埋める。
各スタンドの prompt にこの StandContext を含めることで、情報の欠落を防ぐ。

## Issue コンテキスト

### GitHub Issues

ユーザーが Issue 番号を指定した場合（例: `#239 をやって`）、パイプライン全体で Issue コンテキストを引き回す:

```bash
gh issue view <N> --json title,body,labels,state
```

Issue コンテキストがある場合:
- **ブランチ名**: Issue 番号 + タイトルから自動生成（例: `feat/239-local-dev-setup`）
- **PR リンク**: `Closes #239` を PR body に自動挿入
- **完了時**: パイプラインの最終ステップで `gh issue close` を実行

Issue コンテキストは StandContext に含めて各スタンドに引き継ぐ。

### Linear Issues（オプショナル）

ユーザーが Linear Issue ID を指定した場合（例: `VP-9 をやって`）、Linear MCP が利用可能であれば連携する。
Linear がなくてもパイプラインは動作する（Linear 連携は全てベストエフォート）。

- **Issue 取得**: `get_issue(id: "VP-9")` で内容を把握
- **ステータス更新**: 実装開始時に `save_issue(id: "VP-9", state: "In Progress")`
- **完了時**: `save_issue(id: "VP-9", state: "Done")`
- **Release リンク**: リリース後に `save_issue(id: "VP-9", links: [{url: "リリースURL", title: "Release vX.Y.Z"}])`
- **PR リンク**: PR 作成後に `Closes VP-9` を PR body に含める（Linear の GitHub 連携で自動クローズ）
