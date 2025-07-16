# cypherAES - AES-256 暗号化/復号化 CLI ツール

## 概要

cypherAESは、AES-256暗号化方式を使用したコマンドラインツールです。テキストファイルやバイナリファイルの暗号化・復号化を簡単に行うことができます。

### 学習目的について

このツールは**ITパスポート試験のセキュリティ分野**の学習に最適です。以下の重要な概念を実践的に理解できます：

- **共通鍵暗号方式（AES-256）**の仕組み
- **暗号化キー**の重要性と管理
- **ファイル暗号化**の実際の流れ
- **PKCS7パディング**などの暗号化技術

ITパスポートで出題される暗号化技術を、実際にコマンドラインで操作することで深く理解できます。

## ビルド方法

### 前提条件
- Rust 1.70以上がインストールされていること
- Cargo が利用可能であること

### 依存関係
以下のクレートが必要です（Cargo.tomlに記載）：
```toml
[dependencies]
clap = "4.0"
aes = "0.8"
hex = "0.4"
```

### ビルド手順

1. **リポジトリをクローン**
```bash
git clone <repository-url>
cd cypherAES
```

2. **依存関係をインストール**
```bash
cargo build
```

3. **リリースビルド（推奨）**
```bash
cargo build --release
```

実行ファイルは `target/release/cypherAES`（Windows: `cypherAES.exe`）に生成されます。

## 使用方法

### 基本的な使い方

```bash
cypherAES -m <モード> -k <キーファイル> -f <対象ファイル>
```

### パラメータ

- `-m, --mode`: 動作モード
  - `encrypt`: 暗号化
  - `decrypt`: 復号化
- `-k, --keyfile`: 暗号化キーファイルのパス（32バイト必須）
- `-f, --filepath`: 処理対象ファイルのパス

### キーファイルの準備

**重要**: AES-256では32バイト（256bit）のキーが必要です。

```bash
# 32バイトのランダムキーを生成（Linuxの場合）
dd if=/dev/urandom of=key.txt bs=32 count=1

# Windowsの場合（PowerShell）
$bytes = New-Object byte[] 32
(New-Object System.Security.Cryptography.RNGCryptoServiceProvider).GetBytes($bytes)
[System.IO.File]::WriteAllBytes("key.txt", $bytes)
```

### 使用例

#### 1. テキストファイルの暗号化
```bash
# sample.txtを暗号化
cypherAES -m encrypt -k key.txt -f sample.txt

# 結果: sample.txt.aes が生成される
```

#### 2. ファイルの復号化
```bash
# sample.txt.aesを復号化
cypherAES -m decrypt -k key.txt -f sample.txt.aes

# 結果: sample.txt.aes.dec が生成される
```

#### 3. バイナリファイルの暗号化
```bash
# 画像ファイルを暗号化
cypherAES -m encrypt -k key.txt -f photo.jpg

# 結果: photo.jpg.aes が生成される
```

### ファイル拡張子の変化

| 操作 | 入力ファイル | 出力ファイル | 説明 |
|------|-------------|-------------|------|
| 暗号化 | `sample.txt` | `sample.txt.aes` | 元ファイル名に `.aes` を追加 |
| 復号化 | `sample.txt.aes` | `sample.txt.aes.dec` | 元ファイル名に `.dec` を追加 |

### 注意事項

#### ⚠️ セキュリティに関する重要な注意

1. **キーファイルの管理**
   - キーファイル（key.txt）は厳重に管理してください
   - キーを紛失すると暗号化されたファイルは復号できません
   - キーファイルを他人に渡さないでください

2. **ファイルサイズ**
   - 暗号化後のファイルサイズは16バイト境界に調整されるため、元ファイルより大きくなる場合があります

3. **バイナリファイル**
   - 復号化時にUTF-8変換エラーが表示される場合がありますが、バイナリファイルの場合は正常です

4. **バックアップ**
   - 重要なファイルを暗号化する前に、必ずバックアップを取ってください

#### 📝 制限事項

- 現在はECBモードを使用（将来的にCBCモードに対応予定）
- 復号化結果はUTF-8テキストとして表示されます
- 大容量ファイルの処理には時間がかかる場合があります

## トラブルシューティング

### よくあるエラー

1. **"キーファイルは32バイト(256bit)で指定してください"**
   - キーファイルのサイズが32バイトではありません
   - 正しいサイズのキーファイルを生成してください

2. **"キーファイルの読み込みに失敗しました"**
   - キーファイルのパスが間違っているか、ファイルが存在しません
   - ファイルパスを確認してください

3. **"Failed to read file"**
   - 対象ファイルが見つからないか、読み取り権限がありません
   - ファイルの存在とアクセス権限を確認してください

## ライセンス

MIT License

## 作成者

cacao999

---

# cypherAES - AES-256 Encryption/Decryption CLI Tool

## Overview

cypherAES is a command-line tool that uses AES-256 encryption. It can easily encrypt and decrypt both text files and binary files.

### Educational Purpose

This tool is ideal for learning the **security field of the IT Passport exam**. You can practically understand the following important concepts:

- **Symmetric encryption (AES-256)** mechanisms
- Importance and management of **encryption keys**
- Actual process of **file encryption**
- Encryption technologies such as **PKCS7 padding**

By actually operating encryption technologies covered in the IT Passport exam through command-line operations, you can gain a deep understanding.

## Build Instructions

### Prerequisites
- Rust 1.70 or higher installed
- Cargo available

### Dependencies
The following crates are required (listed in Cargo.toml):
```toml
[dependencies]
clap = "4.0"
aes = "0.8"
hex = "0.4"
```

### Build Steps

1. **Clone the repository**
```bash
git clone <repository-url>
cd cypherAES
```

2. **Install dependencies**
```bash
cargo build
```

3. **Release build (recommended)**
```bash
cargo build --release
```

The executable will be generated at `target/release/cypherAES` (Windows: `cypherAES.exe`).

## Usage

### Basic Usage

```bash
cypherAES -m <mode> -k <keyfile> -f <target_file>
```

### Parameters

- `-m, --mode`: Operation mode
  - `encrypt`: Encryption
  - `decrypt`: Decryption
- `-k, --keyfile`: Path to encryption key file (32 bytes required)
- `-f, --filepath`: Path to target file

### Preparing Key File

**Important**: AES-256 requires a 32-byte (256-bit) key.

```bash
# Generate 32-byte random key (Linux)
dd if=/dev/urandom of=key.txt bs=32 count=1

# Windows (PowerShell)
$bytes = New-Object byte[] 32
(New-Object System.Security.Cryptography.RNGCryptoServiceProvider).GetBytes($bytes)
[System.IO.File]::WriteAllBytes("key.txt", $bytes)
```

### Examples

#### 1. Encrypt Text File
```bash
# Encrypt sample.txt
cypherAES -m encrypt -k key.txt -f sample.txt

# Result: sample.txt.aes is generated
```

#### 2. Decrypt File
```bash
# Decrypt sample.txt.aes
cypherAES -m decrypt -k key.txt -f sample.txt.aes

# Result: sample.txt.aes.dec is generated
```

#### 3. Encrypt Binary File
```bash
# Encrypt image file
cypherAES -m encrypt -k key.txt -f photo.jpg

# Result: photo.jpg.aes is generated
```

### File Extension Changes

| Operation | Input File | Output File | Description |
|-----------|------------|-------------|-------------|
| Encryption | `sample.txt` | `sample.txt.aes` | Adds `.aes` to original filename |
| Decryption | `sample.txt.aes` | `sample.txt.aes.dec` | Adds `.dec` to original filename |

### Important Notes

#### ⚠️ Critical Security Notes

1. **Key File Management**
   - Manage the key file (key.txt) strictly
   - If you lose the key, encrypted files cannot be decrypted
   - Do not share the key file with others

2. **File Size**
   - Encrypted file size may be larger than the original due to 16-byte boundary alignment

3. **Binary Files**
   - UTF-8 conversion errors may be displayed during decryption, but this is normal for binary files

4. **Backup**
   - Always create backups before encrypting important files

#### 📝 Limitations

- Currently uses ECB mode (CBC mode support planned for future)
- Decryption results are displayed as UTF-8 text
- Processing large files may take time

## Troubleshooting

### Common Errors

1. **"Key file must be specified as 32 bytes (256bit)"**
   - Key file size is not 32 bytes
   - Generate a key file with the correct size

2. **"Failed to read key file"**
   - Key file path is incorrect or file doesn't exist
   - Check the file path

3. **"Failed to read file"**
   - Target file not found or no read permission
   - Check file existence and access permissions

## License

MIT License

## Author

cacao999