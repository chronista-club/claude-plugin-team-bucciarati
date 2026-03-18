# MCP Tools Reference

Team Bucciarati の各スタンドが活用できる MCP ツール。
全てオプショナル — ツールが利用不可でもスタンドは動作する。

## gitnexus — コードベースナレッジグラフ

Tree-sitter でコードを解析し、依存関係・呼び出しチェーン・シンボル関係をグラフ化。

### セットアップ
```bash
claude mcp add gitnexus -- bunx gitnexus mcp
```

### 主なツール
| ツール | 用途 |
|--------|------|
| `search_codebase` | コードベース全体をセマンティック検索 |
| `get_symbol_details` | シンボルの詳細（型、依存、呼び出し元/先） |
| `get_dependencies` | 依存関係グラフの取得 |
| `get_call_chain` | 関数の呼び出しチェーンを追跡 |

### スタンド別活用
- **Purple Haze**: 広域リサーチで依存グラフを辿り、影響範囲を特定
- **Moody Blues**: diff 内シンボルの依存先を確認、変更の影響範囲を検証
- **Spice Girl**: テスト対象の依存関係を把握し、テスト設計に反映

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
- **Moody Blues**: 変更シンボルの参照元を確認、リグレッションリスク評価
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
- **Spice Girl**: テストフレームワークの API・パターンを確認

## linear — Issue 管理

Linear の Issue・プロジェクト・サイクルを操作。

### 主なツール
| ツール | 用途 |
|--------|------|
| `get_issue` | Issue の詳細取得 |
| `save_issue` | Issue の作成・更新 |
| `list_issues` | Issue 一覧 |
| `get_issue_status` | ステータス確認 |

### スタンド別活用
- **Aerosmith**: Issue Pipeline で Issue コンテキストの取得・ステータス管理
- **Sticky Fingers**: PR 作成時にステータスを "In Progress" に、マージ後に "Done" に
