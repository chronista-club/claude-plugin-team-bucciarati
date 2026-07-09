# Pipeline Patterns

Team Bucciarati のパイプラインパターン。シンプルに4+1構成。

**全パイプラインの終点は「コミット可能な working tree」。** コミット・PR・デプロイは含めない。

## Finish（デフォルト）

日常の 80% がこれ。手元の変更をコミット可能な品質に仕上げる。

```
(Spice Girl) → Moody Blues
  テスト補強(任意) →  品質検証
```

### フロー

1. **Spice Girl**（任意）: 変更箇所のテストが手薄なら補強
2. **Moody Blues**: ローカルチェック + コードレビュー + lint/format 自動修正
   - BLOCKED → パイプライン停止
   - NEEDS WORK → ユーザーに修正を促して再実行
   - COMMIT READY → 完了報告

### トリガー例
- 「この変更、仕上げて」
- 「コミットできる状態にして」
- `/dispatch`（引数なし、手元に diff がある場合）

## Forge

要件から実装一式。調査 → 実装 → テスト → レビュー。

```
(Purple Haze) → Gold Experience → Spice Girl → Moody Blues
  調査(任意)  →     実装       →  テスト強化  →  品質検証
```

### フロー

1. **Purple Haze**（任意）: 実装の背景調査、影響範囲・既存パターンの調査
2. **Gold Experience**: 要件理解 → 実装 → ローカル検証（全 green まで）
3. **Spice Girl**: 境界値・異常系テストの追加
4. **Moody Blues**: ローカルチェック + 多角的コードレビュー

### トリガー例
- 「この機能、実装からレビューまで全部やって」
- 「VP-12 を実装して仕上げて」
- `/dispatch forge`

## Polish

挙動を変えずに構造を美しくする。

```
Sticky Fingers → Moody Blues
  リファクタ   →  品質検証
```

### フロー

1. **Sticky Fingers**: 安全網確認 → 一手ずつ分解・再結合（テストがなければ Spice Girl を先に挟む）
2. **Moody Blues**: 挙動変更が混入していないか最終検証

### トリガー例
- 「このモジュールきれいにして」
- 「リファクタして」
- `/dispatch polish`

## Barrage

独立した複数のコード作業を並列一斉実行。

```
Sex Pistols → Moody Blues
  並列作業   →  品質検証
```

### フロー

1. **Sex Pistols**: タスク分解 → 並列サブエージェント実行 → 統合検証
2. **Moody Blues**: 合成後の diff 全体をレビュー

### トリガー例
- 「この API 移行、全ファイル一斉にやって」
- `/dispatch barrage`

## Custom

上記に当てはまらない場合、スタンドを自由に組み合わせる。

### 例

| パターン | フロー | ユースケース |
|---------|--------|------------|
| 調査 → テスト | Purple Haze → Spice Girl | 調査してからテスト設計 |
| テスト + レビュー | Spice Girl → Moody Blues | テスト追加してレビューのみ |
| リファクタ → 実装 | Sticky Fingers → Gold Experience | 構造を整えてから機能を生やす |
| 調査のみ | Purple Haze | 技術調査・根本原因特定 |

### トリガー例
- 「先に構造整えてから実装して」→ Sticky Fingers → Gold Experience → Moody Blues
- `/dispatch custom`

## 単体呼び出し

パイプラインを組まず、スタンドを直接呼ぶ:

| 呼び方 | スタンド |
|--------|---------|
| 「レビューして」 | Moody Blues 直接 |
| 「調べて」 | Purple Haze 直接 |
| 「テスト書いて」 | Spice Girl 直接 |
| 「実装して」 | Gold Experience 直接 |
| 「リファクタして」 | Sticky Fingers 直接 |
| 「並列でやって」 | Sex Pistols 直接 |

> 1スタンドで完結する場合はパイプラインを組む必要なし。

## パイプライン途中再開

パイプラインが途中で停止した場合（Moody Blues が BLOCKED、テスト失敗等）、修正後に途中から再開:

```
/dispatch resume
```

### 再開フロー

1. 前回の停止ポイントを確認（git status、テスト結果、前回レポート）
2. 停止原因が解消されているか検証
3. 停止したステップから再開（最初からやり直さない）

### 再開可能な停止パターン

| 停止原因 | 再開ポイント |
|---------|-------------|
| Moody Blues BLOCKED (checks fail) | 修正後、Moody Blues から再実行 |
| Gold Experience 検証 fail | Gold Experience の検証から |
| Sticky Fingers 安全網なし | Spice Girl でテスト追加後、Sticky Fingers から |
