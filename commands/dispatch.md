---
description: "Aerosmith を起動してパイプラインをディスパッチする"
---

Aerosmith エージェントを起動してください。

ユーザーの現在の状況（git status, diff, log）を偵察し、最適なパイプラインを提案してください。

## 引数の解釈

- **パイプライン名が指定された場合**: そのパイプラインを直接実行
- **`resume`**: 前回停止したパイプラインの途中から再開（git log、PR 状態、CI 結果から停止ポイントを特定）
- **指定なし**: 以下の選択肢を提示

## パイプライン選択肢

1. **Full Release** — 調査 → レビュー → シップ → デプロイ
2. **Review & Ship** — レビュー → シップ
3. **Ship & Deploy** — シップ → デプロイ
4. **Test & Ship** — テスト → レビュー → シップ
5. **Research Only** — 調査のみ
6. **Deploy Only** — デプロイのみ
7. **Parallel Sprint** — 並列ワーカー
8. **Issue Pipeline** — Issue 起点のエンドツーエンド

$ARGUMENTS
