# MCP Tools Reference

Team Bucciarati の各スタンドが活用できる MCP ツール。
全てオプショナル — ツールが利用不可でもスタンドは動作する。

## gitnexus — コードベースナレッジグラフ

Tree-sitter でコードを解析し、依存関係・呼び出しチェーン・シンボル関係をグラフ化。
事前に `gitnexus analyze` でリポジトリをインデックスする必要がある。

### セットアップ
```bash
bun add -g gitnexus
node ~/.bun/install/global/node_modules/@ladybugdb/core/install.js  # postinstall 手動実行
claude mcp add --scope user gitnexus -- gitnexus mcp
gitnexus analyze  # リポジトリのインデックス作成
```

### ツール一覧
| ツール | 用途 |
|--------|------|
| `query` | 自然言語/キーワードで実行フローを検索（BM25 + セマンティック） |
| `context` | シンボルの360度ビュー（呼び出し元/先、参照、プロセス参加） |
| `impact` | 変更の blast radius 分析（depth 別リスク分類） |
| `detect_changes` | git diff から影響を受ける実行フローを特定 |
| `cypher` | Cypher クエリでナレッジグラフを直接検索 |
| `rename` | グラフベースの安全なリネーム（プレビュー付き） |
| `list_repos` | インデックス済みリポジトリ一覧 |

### スタンド別活用
- **Purple Haze**: `query` で全体像把握 → `context` でシンボル深堀り → `impact` で影響範囲特定
- **Gold Experience**: `query` / `context` で実装地点の周辺構造・既存パターンを把握
- **Moody Blues**: `detect_changes` で diff の影響フロー検出 → `impact` で変更シンボルの blast radius 確認
- **Sticky Fingers**: `rename` で安全なリネーム、`impact` で分解前の blast radius 確認
- **Sex Pistols**: `impact` で依存関係を分析し、安全な並列分割境界を見つける
- **Spice Girl**: `context` でテスト対象の依存関係把握 → `query` でテスト対象の実行フロー理解

## serena — シンボリックコード解析

コードのシンボル構造を理解し、精密な読み書きを行う。

### 主なツール
| ツール | 用途 |
|--------|------|
| `find_symbol` | シンボルを名前パスで検索 |
| `get_symbols_overview` | ファイル内のシンボル一覧 |
| `find_referencing_symbols` | シンボルの参照元を検索 |
| `replace_symbol_body` | シンボルの定義を置換 |

### スタンド別活用
- **Purple Haze**: シンボルの依存関係を辿って設計意図を理解
- **Gold Experience**: 実装地点のシンボル構造を正確に把握して精密に追記
- **Moody Blues**: 変更シンボルの参照元を確認、リグレッションリスク評価
- **Sticky Fingers**: `find_referencing_symbols` で参照元を洗い出し、`replace_symbol_body` でシンボル単位の精密な組み替え
- **Spice Girl**: テスト対象の公開 API とメソッドシグネチャを正確に把握

## context7 — ライブラリドキュメント

ライブラリの最新ドキュメントとコード例を取得。

### 主なツール
| ツール | 用途 |
|--------|------|
| `resolve-library-id` | ライブラリ ID の解決 |
| `query-docs` | ドキュメント・コード例の取得 |

### スタンド別活用
- **Purple Haze**: 技術調査でライブラリの API 仕様・ベストプラクティスを確認
- **Gold Experience**: 使用ライブラリの最新 API・推奨パターンを確認してから実装
- **Spice Girl**: テストフレームワークの API・パターンを確認

## vantage-point — Canvas 可視化（Aerosmith Radar）

VP Canvas にパイプラインの作戦盤を描く。詳細な運用は [vp-canvas.md](vp-canvas.md) を参照。

### 主なツール
| ツール | 用途 |
|--------|------|
| `show` | Radar 盤面の表示・更新（markdown、同一 title で再 show = 更新） |
| `capture_canvas` | mission 完了時の盤面キャプチャ（作戦記録） |

### スタンド別活用
- **Aerosmith のみ** — 各スタンドは Canvas を触らない（責務分離）。`SP 未接続` エラー時は以降スキップ

## linear — Issue 参照（読み取りのみ）

Linear の Issue を**要件ソースとして読み取る**。ステータス更新・クローズは行わない（コミットライン以降は team-b の領分外）。

### 主なツール
| ツール | 用途 |
|--------|------|
| `get_issue` | Issue の詳細取得（要件・受け入れ条件の把握） |
| `list_issues` | Issue 一覧 |

### スタンド別活用
- **Aerosmith**: パイプライン開始時に Issue コンテキストを取得し、StandContext で引き回す
- **Gold Experience**: 実装対象の要件・受け入れ条件を Issue から読み取る
