/*!
 * libs.rs - AES-256暗号化/復号化ライブラリ
 * 
 * 作成者: cacao999
 * 作成日: 2024年
 * 概要: AES-256-ECBモードによる暗号化・復号化機能を提供
 *       PKCS7パディングを使用してブロック暗号化を実装
 * 
 * 注意事項:
 *   - 現在はECBモードを使用（CBCモードへの移行を推奨）
 *   - IVの実装が必要（セキュリティ強化のため）
 *   - バイナリファイルの復号化時はUTF-8変換に注意
 * 
 * 依存関係:
 *   - aes: AES暗号化アルゴリズム実装
 *   - cipher: 暗号化プリミティブ
 * 
 * TODO:
 *   - CBCモードの実装
 *   - IV（初期化ベクトル）の追加
 *   - エラーハンドリングの改善
 */

use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes256;

/**
 * AES-256による暗号化処理
 * 
 * PKCS7パディングを適用してブロック暗号化を実行
 * 
 * @param plaintext 暗号化対象のプレーンテキスト
 * @param key AES-256暗号化キー（32バイト）
 * @return 暗号化されたデータ
 * 
 * 処理フロー:
 *   1. 入力データをPKCS7パディングで16バイト境界に調整
 *   2. 16バイトブロックごとにAES-256暗号化を実行
 *   3. 暗号化されたブロックを連結して返却
 */
pub fn encrypt_aes256(plaintext: &[u8], key: &[u8; 32]) -> Vec<u8> {
    // AES-256暗号化器の初期化
    let cipher = Aes256::new(GenericArray::from_slice(key));

    // プレーンテキストを16バイトブロックにパディング
    let mut padded_text = plaintext.to_vec();

    // PKCS7パディングを実装
    // 16バイトブロックに満たない部分をパディング値で埋める
    let padding_len = 16 - (padded_text.len() % 16);
    for _ in 0..padding_len {
        padded_text.push(padding_len as u8);
    }

    let mut encrypted = Vec::new();

    // 16バイトずつ暗号化処理
    // AES-256はブロック暗号のため、16バイト単位での処理が必要
    for chunk in padded_text.chunks(16) {
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.encrypt_block(&mut block);
        encrypted.extend_from_slice(&block);
    }

    encrypted
}

/**
 * AES-256による復号化処理
 * 
 * 暗号化されたデータを復号化し、PKCS7パディングを除去
 * 
 * @param ciphertext 復号化対象の暗号化データ
 * @param key AES-256復号化キー（32バイト）
 * @return 復号化されたテキスト（UTF-8文字列）
 * 
 * 処理フロー:
 *   1. 16バイトブロックごとにAES-256復号化を実行
 *   2. PKCS7パディングを除去
 *   3. UTF-8文字列に変換して返却
 * 
 * 注意: バイナリファイルの場合、UTF-8変換でエラーが発生する可能性あり
 */
pub fn decrypt_aes256(ciphertext: &[u8], key: &[u8; 32]) -> String {
    // AES-256復号化器の初期化
    let cipher = Aes256::new(GenericArray::from_slice(key));

    let mut decrypted = Vec::new();

    // 16バイトずつ復号化処理
    // 暗号化時と同様に16バイト単位での処理が必要
    for chunk in ciphertext.chunks(16) {
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.decrypt_block(&mut block);
        decrypted.extend_from_slice(&block);
    }

    // PKCS7パディングを削除
    // 最後のバイトがパディング長を示している
    if let Some(&padding_len) = decrypted.last() {
        // パディング長の妥当性チェック
        if padding_len > 0 && padding_len <= 16 {
            let len = decrypted.len();
            decrypted.truncate(len - padding_len as usize);
        }
    }

    // バイト配列をUTF-8文字列に変換
    // TODO: バイナリファイル対応のため、Vec<u8>を返すバージョンも検討
    String::from_utf8(decrypted).unwrap_or_else(|_| {
        eprintln!("警告: UTF-8変換に失敗しました。バイナリファイルの可能性があります。");
        "Invalid UTF-8".to_string()
    })
}