# Pipeline Patterns

Team Bucciarati のパイプラインパターン詳細定義。

## Full Release

最も包括的なパイプライン。新機能のフルリリースに使用。

```
Purple Haze → Moody Blues → Sticky Fingers → Gold Experience
  調査(任意) →  品質検証  →  シッピング   →   デプロイ
```

### フロー

1. **Purple Haze (Research)**: 実装の背景調査、関連コードの影響範囲を調査
2. **Moody Blues (Quality Gate)**: CI チェック + 多角的コードレビュー
   - BLOCKED → パイプライン停止
   - NEEDS WORK → ユーザーに修正を促して再実行
   - SHIP IT → 次へ
3. **Sticky Fingers (Shipping)**: コミット → プッシュ → PR作成 → CI確認 → マージ
4. **Gold Experience (Deploy)**: ビルド → マイグレーション → デプロイ → ヘルスチェック

### トリガー例
- 「フルリリースして」
- 「レビューからデプロイまで全部やって」
- 「完全なパイプラインで」

## Review & Ship

コードレビュー後にシッピングまで行う標準パターン。

```
Moody Blues → Sticky Fingers
  品質検証  →  シッピング
```

### フロー

1. **Moody Blues**: CI + コードレビュー
2. **Sticky Fingers**: コミット → PR → マージ

### トリガー例
- 「レビューしてシップして」
- 「チェックしてからPR出して」

## Ship & Deploy

既にレビュー済みの変更をシップしてデプロイ。

```
Sticky Fingers → Gold Experience
  シッピング   →   デプロイ
```

### フロー

1. **Sticky Fingers**: コミット → PR → マージ
2. **Gold Experience**: ビルド → デプロイ → ヘルスチェック

### トリガー例
- 「シップしてデプロイまで」
- 「PR出してからデプロイして」

## Test & Ship

テストを強化してからシップする品質重視パターン。

```
Spice Girl → Moody Blues → Sticky Fingers
  テスト生成 →  品質検証  →  シッピング
```

### フロー

1. **Spice Girl**: テストリスト設計 → テスト実装 → テスト実行
2. **Moody Blues**: CI + コードレビュー（テスト込み）
3. **Sticky Fingers**: コミット → PR → マージ

### トリガー例
- 「テスト書いてからシップして」
- 「テストカバレッジ上げてからPR出して」

## Research Only

調査のみ。コード変更なし。

```
Purple Haze
  調査
```

### フロー

1. **Purple Haze**: 調査 → レポート生成

### トリガー例
- 「このバグの原因を調べて」
- 「ベストプラクティスを調査して」

## Deploy Only

マージ済みのコードをデプロイ。

```
Gold Experience
  デプロイ
```

### フロー

1. **Gold Experience**: ビルド → マイグレーション → デプロイ → ヘルスチェック

### トリガー例
- 「本番にデプロイして」
- 「デプロイしてヘルスチェックまで」

## Parallel Sprint

複数タスクを並列実行。大規模リファクタリングや複数Issue同時進行に使用。

```
Sex Pistols
  並列ワーカー管理
```

### フロー

1. **Sex Pistols**: タスク分解 → ワーカー生成 → タスクディスパッチ → 進捗監視 → 結果収集

### トリガー例
- 「この3つのIssueを並列で」
- 「ワーカー立てて並列でやって」

## Issue Pipeline

Issue 番号を起点にしたエンドツーエンドパイプライン。Issue → Branch → 実装 → PR → Merge → Deploy → Close。

```
Issue #N → Sticky Fingers → (Moody Blues →) Gold Experience → Issue Close
  起点   →  ブランチ+PR  → (品質検証 →)    デプロイ       →  完了
```

### フロー

1. **Aerosmith**: `gh issue view #N` で Issue 内容を把握、Issue コンテキストを生成
2. **Sticky Fingers**: Issue 番号からブランチ作成（`feat/N-slug`）、PR に `Closes #N` 挿入、マージ
3. **Moody Blues** (任意): CI + レビュー（変更規模に応じて省略可）
4. **Gold Experience** (任意): デプロイが必要な場合のみ
5. **Issue Close**: マージ時の `Closes #N` で自動クローズ、またはデプロイ後に手動クローズ

### トリガー例
- 「#239 をやって」
- 「Issue 239 をフルパイプラインで」
- 「この Issue、ブランチ切ってPR出してマージまで」

### Issue コンテキスト

各スタンドに引き渡される情報:
- `issue_number`: GitHub Issue 番号
- `issue_title`: Issue タイトル
- `branch_name`: Issue から生成されたブランチ名
- `needs_deploy`: デプロイが必要かどうか（ラベル等から判断）

## カスタムパイプライン

上記パターンに当てはまらない場合、Aerosmith が状況に応じてカスタムパイプラインを構築する。

例:
- `Spice Girl → Moody Blues` — テスト追加 + レビューのみ（シップしない）
- `Purple Haze → Spice Girl` — 調査してからテスト設計
- `Sex Pistols → Moody Blues → Sticky Fingers` — 並列実装 → まとめてレビュー → シップ
