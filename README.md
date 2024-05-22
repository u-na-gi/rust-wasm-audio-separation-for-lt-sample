# rust-wasm-audio-separation-for-lt-sample


# Rust-WASM 音声分離サンプル

このプロジェクトは、RustとWebAssembly (WASM) を使用したスピーカー分離の実装例を示します。ブラウザで動作するサンプルアプリケーションで、音声ストリームをスピーカーごとに分離します。

## 特徴

- **Rust**: パフォーマンスと安全性のためにRustで実装されたコアロジック。
- **WASM**: RustコードをWebAssemblyにコンパイルしてブラウザで実行。
- **ウェブインターフェース**: 音声ファイルの生成と処理のためのシンプルなウェブインターフェース。


## 必要条件

- RustおよびCargo
- Node.js
- wasm-pack
- Python

## インストール

1. リポジトリをクローン:
   ```sh
   git clone https://github.com/u-na-gi/rust-wasm-audio-separation-for-lt-sample.git
   ```
2. プロジェクトディレクトリに移動:
   ```sh
   cd rust-wasm-audio-separation-for-lt-sample
   ```
3. 依存関係をインストール:
   ```sh
   npm install
   ```
4. 音声ファイルを生成する:
   ```sh
   python create_resource/main.py
   ```
5. プロジェクトをビルド:
   ```sh
   make
   ```
6. サーバーを起動:
   ```sh
   npm run dev
   ```

## 使用方法

1. ブラウザを開き、`http://localhost:8080`にアクセス。
2. 音声ファイルを生成するPythonスクリプトを実行:
   ```sh
   python create_resource/main.py
   ```
3. 「プロセス」をクリックして、スピーカーごとに音声を分離。

## WASMのテスト

```
make test
```

firefoxで開いてください。

## ライセンス

このプロジェクトはMITライセンスの下でライセンスされています。