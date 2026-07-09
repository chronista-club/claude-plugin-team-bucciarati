# StandContext 仕様

## 構造

パイプライン内で各スタンド間に引き継がれるコンテキスト。
各スタンドの結果を次のスタンドに渡す際、以下の構造化フォーマットを使用する:

```
## StandContext

### Source
stand: <前のスタンド名>
status: <DONE / COMMIT READY / NEEDS WORK / BLOCKED / ERROR>

### Artifacts
diff_summary: <+X -Y (N files)>
files: <主要な変更ファイル>
tests_status: <PASS / FAIL / NONE>
checks_status: <PASS / FAIL>（typecheck / lint / build）

### Issue
type: <linear / github>
id: <Linear ID or Issue 番号>
title: <Issue タイトル>

### Notes
<前のスタンドからの引き継ぎメモ — 設計判断、発見したバグ、次スタンドへの依頼>
```

**全てのフィールドはオプショナル。** 該当するものだけ埋める。
各スタンドの prompt にこの StandContext を含めることで、情報の欠落を防ぐ。

## Issue コンテキスト

Issue は**要件のソースとして読み取るだけ**。ステータス更新・クローズ・PR リンクは team-b の領分外（コミットライン以降）。

### Linear Issues（デフォルト）

ユーザーが Linear Issue ID を指定した場合（例: `VP-9 をやって`）:

- **Issue 取得**: `get_issue(id: "VP-9")` で要件・受け入れ条件を把握
- 取得した内容を StandContext の `Issue` セクションに含めて各スタンドに引き継ぐ
- **書き込みはしない** — `save_issue` によるステータス変更・クローズは行わない
- Linear MCP が使えない場合はスキップ（パイプラインは止めない）

### GitHub Issues

GitHub Issues が有効なリポジトリの場合:

- `gh issue view <N>` で要件を読み取る
- クローズ・コメント投稿はしない
