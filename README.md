# ToDo管理Webアプリ

メールアドレス認証付きの ToDo 管理 Web アプリケーションです。

フロントエンド、バックエンド、データベースを分離した構成で、日々のタスク管理を行うための基本機能を実装しています。ポートフォリオとして、認証、CRUD、状態管理、検索性を意識した一覧機能までを一通り揃えています。

## 概要

- フロントエンドは `Next.js` と `TypeScript` を用いて実装
- バックエンドは `Rust + Axum` を用いて REST API を構築
- データベースは `PostgreSQL` を採用
- 認証はメールアドレス + パスワード方式
- アクセストークンとリフレッシュトークンを用いたセッション管理を実装

## 主な機能

### 認証機能

- ユーザー新規登録
- ログイン / ログアウト
- アクセストークン更新
- 認証済みユーザー情報の取得

### ToDo管理機能

- ToDo の作成 / 一覧表示 / 編集 / 削除
- ステータス管理（未着手 / 進行中 / 完了）
- 優先度管理（低 / 中 / 高）
- 期限日時の設定
- ステータス・優先度による絞り込み
- 作成日 / 期限日 / 優先度でのソート
- ページネーション対応

### 開発・保守性

- OpenAPI による API ドキュメント生成
- Swagger UI による API 確認
- フロントエンド / バックエンドそれぞれにテストを用意
- Docker Compose による開発環境構築

## システム構成

| レイヤー | 技術 | 役割 |
| --- | --- | --- |
| フロントエンド | Next.js 15 / TypeScript / Radix UI Themes / Tailwind CSS 4 | 画面表示、フォーム入力、認証状態の管理 |
| バックエンド | Rust / Axum / sqlx | 認証 API、ToDo API、業務ロジック |
| データベース | PostgreSQL 17 | ユーザー、リフレッシュトークン、ToDo データの永続化 |
| 開発環境 | Docker Compose | フロントエンド、バックエンド、DB の統合起動 |

## 技術スタック

### バックエンド

| カテゴリ | 技術 | バージョン |
| --- | --- | --- |
| Webフレームワーク | Axum | 0.8.x |
| 非同期ランタイム | Tokio | 1.x |
| DBドライバ | sqlx | 0.8.x |
| JWT認証 | jsonwebtoken | 10.x |
| パスワードハッシュ化 | bcrypt | 0.16.x |
| OpenAPI | utoipa + utoipa-swagger-ui | 5.x / 9.x |
| HTTPクライアント | reqwest | 0.12.x |
| OAuth 2.0 | oauth2 | 4.x |

### フロントエンド

| カテゴリ | 技術 | バージョン |
| --- | --- | --- |
| フレームワーク | Next.js | 15.x |
| UIライブラリ | Radix UI Themes | 3.x |
| CSS | Tailwind CSS | 4.x |
| 言語 | TypeScript | 5.x |
| フォーム管理 | react-hook-form | 7.x |
| バリデーション | zod | 4.x |
| HTTPクライアント | axios | 1.x |

## アプリケーション構成

- `frontend/`
  - Next.js ベースのフロントエンドアプリケーション
  - ログイン画面、新規登録画面、ToDo 一覧画面を提供
- `backend/`
  - Axum ベースの API サーバー
  - 認証 API と ToDo CRUD API を提供
- `compose.yaml`
  - フロントエンド、バックエンド、PostgreSQL をまとめて起動するための設定

## 補足

- API ドキュメントは Swagger UI で確認できます
- 開発環境では `docker compose` を使って各サービスを起動できます

## 利用OSS

- wait-for-it / https://github.com/vishnubob/wait-for-it
