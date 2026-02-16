# Actix-web + SQLx + PostgreSQL で作る Todo API ロードマップ

目的：

Rustで **Web API開発（actix-web + SQLx + PostgreSQL）の実践力を体系的に身につけること**

技術構成：

- Web: actix-web
- DB: PostgreSQL
- DBアクセス: SQLx
- 非同期ランタイム: Tokio
- マイグレーション: sqlx-cli

---

# フェーズ0: 前提整理（Rust基礎）

## STEP-00-01: Rust asyncの理解

学習内容：

- async / await
- Futureの概念
- Tokioランタイム
- `.await` が必要な理由
- 非同期I/Oの基本的な流れ

到達目標：

- 非同期関数の実行フローを説明できる
- なぜ actix-web が async ベースなのか理解できる

---

# フェーズ1: 環境構築

## STEP-01-01: プロジェクト作成

実施内容：

```bash
cargo new todo-api
cd todo-api
```

Cargo.toml へ追加：

```toml
[dependencies]
actix-web = "4"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres", "macros"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
dotenvy = "0.15"
```

到達目標：

- `cargo run` でサーバーが起動する最小構成を作れる

---

## STEP-01-02: PostgreSQL準備

実施内容：

- PostgreSQLインストール
- データベース作成

```sql
CREATE DATABASE todo_db;
```

到達目標：

- psqlでDB接続できる
- データベースの作成・削除ができる

---

## STEP-01-03: sqlx-cli導入

インストール：

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

環境変数設定：

```bash
export DATABASE_URL=postgres://user:password@localhost/todo_db
```

DB作成確認：

```bash
sqlx database create
```

到達目標：

- sqlx CLIがDBと接続できる
- DATABASE_URLの意味を理解できる

---

# フェーズ2: DB設計

## STEP-02-01: テーブル設計

Todoテーブル定義：

```sql
CREATE TABLE todos (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
```

学習ポイント：

- SERIALの意味
- PRIMARY KEY
- DEFAULT
- TIMESTAMP

到達目標：

- 各カラムの役割を説明できる

---

## STEP-02-02: マイグレーション管理

マイグレーション作成：

```bash
sqlx migrate add create_todos
```

マイグレーション適用：

```bash
sqlx migrate run
```

到達目標：

- migrateの仕組みを理解する
- バージョン管理の意味を説明できる

---

# フェーズ3: SQLx基礎実装

## STEP-03-01: Connection Pool作成

実施内容：

- PgPool生成
- AppStateに保持
- actix-webに共有

学習ポイント：

- なぜ毎回接続しないのか
- プールの役割
- 非同期接続管理

到達目標：

- 接続プールの役割を説明できる

---

## STEP-03-02: SELECT実装（一覧取得）

エンドポイント：

- GET /todos

学習内容：

- `query_as!` マクロ
- structとDBのマッピング
- JSONレスポンス化

到達目標：

- 全件取得APIが動く

---

## STEP-03-03: INSERT実装（作成）

エンドポイント：

- POST /todos

学習内容：

- JSON受信
- INSERT文実装
- RETURNING句

到達目標：

- Todoが追加できる

---

## STEP-03-04: UPDATE実装（完了更新）

エンドポイント：

- PUT /todos/{id}

学習内容：

- パスパラメータ取得
- UPDATE文
- 影響行数の確認

到達目標：

- completed更新ができる

---

## STEP-03-05: DELETE実装（削除）

エンドポイント：

- DELETE /todos/{id}

学習内容：

- DELETE文
- 存在チェック

到達目標：

- 削除APIが動作する

---

# フェーズ4: アーキテクチャ整理

## STEP-04-01: レイヤ分離

ディレクトリ構造：

```
src/
 ├ main.rs
 ├ handlers/
 ├ models/
 ├ repository/
 ├ db/
```

目的：

- handlerとDBロジックの分離
- 単一責任の原則理解

到達目標：

- handlerにSQLを書かない構造にできる
- 責務の分離を説明できる

---

## STEP-04-02: エラーハンドリング整理

導入：

- thiserror

学習内容：

- カスタムエラー定義
- Result型統一
- HTTPステータスへの変換

到達目標：

- DBエラーが適切にHTTPレスポンス化される

---

# フェーズ5: 実践強化

## STEP-05-01: バリデーション追加

内容：

- titleの長さ制限
- 空文字チェック
- 不正リクエスト対応

到達目標：

- バリデーションエラーを適切に返せる

---

## STEP-05-02: ページネーション実装

内容：

- LIMIT / OFFSET
- クエリパラメータ取得

到達目標：

- 大量データに対応できるAPI設計を理解

---

## STEP-05-03: インデックス追加

```sql
CREATE INDEX idx_todos_created_at ON todos(created_at);
```

目的：

- パフォーマンス理解
- EXPLAINの確認

到達目標：

- インデックスの意味を説明できる

---

## STEP-05-04: トランザクション理解

学習内容：

```rust
let mut tx = pool.begin().await?;
```

- commit
- rollback
- エラー時の挙動

到達目標：

- トランザクションの必要性を説明できる

---

# フェーズ6: 実務レベル拡張（任意）

## STEP-06-01: ユーザー追加（1対多リレーション）

## STEP-06-02: JWT認証追加

## STEP-06-03: Docker化

## STEP-06-04: 本番ビルド最適化

---

# 最終到達目標

- async Web APIの理解
- SQLxの型安全理解
- PostgreSQL操作理解
- レイヤ分離理解
- 接続プール理解
- トランザクション理解
- エラーハンドリング設計理解
- 実務レベルAPI構築力

---
