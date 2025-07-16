/*!
 * cypherAES - AES-256-CBC 暗号化/復号化 CLI ツール
 * 
 * 作成者: cacao999
 * 作成日: 2024年
 * 概要: AES-256-CBC方式による暗号化・復号化を行うコマンドラインツール
 *       テキストファイルおよびバイナリファイルの両方に対応
 * 
 * 使用方法:
 *   暗号化: ./cypherAES -m encrypt -k キーファイル -f 対象ファイル
 *   復号化: ./cypherAES -m decrypt -k キーファイル -f 暗号化ファイル
 * 
 * 依存関係:
 *   - clap: コマンドライン引数解析
 *   - hex: 16進数エンコード/デコード
 *   - libs: 独自AES暗号化/復号化モジュール
 * 
 * ライセンス: MIT
 * バージョン: 1.0.0
 */

use clap::{Arg, Command};
use hex;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process;

mod libs;

fn main() {
    // コマンドライン引数の定義
    // clap crateを使用してCLIインターフェースを構築
    let matches = Command::new("cypherAES")
        .about("AES-256-CBC Encrypt/Decrypt CLI Tool")
        .arg(
            Arg::new("mode")
                .short('m')
                .long("mode")
                .help("Mode: encrypt or decrypt")
                .required(true)
                .value_parser(["encrypt", "decrypt"]) // encryptかdecryptのみ許可
                .value_name("MODE"),
        )
        .arg(
            Arg::new("keyfile")
                .short('k')
                .long("keyfile")
                .help("Path to the 32-byte key file")
                .required(true)
                .value_name("KEYFILE"),
        )
        .arg(
            Arg::new("filepath")
                .short('f')
                .long("filepath")
                .help("file to be encripted") // TODO: typo修正 "encrypted"
                .required(true)
                .value_name("FILEPATH"),
        )
        .get_matches();

    // コマンドライン引数の取得と検証
    let mode = matches
        .get_one::<String>("mode")
        .expect("モードが取得できません");

    // キーファイルの読み込みと検証
    // AES-256では32バイト（256bit）のキーが必要
    let keyfile = matches
        .get_one::<String>("keyfile")
        .expect("キーファイル名が取得できません");
    
    let key_bytes = fs::read(keyfile).expect("キーファイルの読み込みに失敗しました");

    // キーサイズの検証（AES-256は32バイト固定）
    if key_bytes.len() != 32 {
        eprintln!("エラー: キーファイルは32バイト(256bit)で指定してください");
        process::exit(1);
    }

    // Vec<u8>から[u8; 32]への変換
    // AES暗号化関数で固定長配列が必要なため
    let key_bytes: [u8; 32] = key_bytes
        .try_into()
        .expect("32バイト配列に変換できません");

    // 対象ファイルの読み込み
    // バイナリファイルも対応するためVec<u8>で読み込み
    let file_path = matches
        .get_one::<String>("filepath")
        .expect("対象ファイルが読み込めません");
    
    let inputed_data = fs::read(file_path).expect("Failed to read file");

    // 暗号化/復号化の処理分岐
    match mode.as_str() {
        "encrypt" => {
            handle_encryption(&inputed_data, &key_bytes, file_path);
        }
        "decrypt" => {
            handle_decryption(&inputed_data, &key_bytes, file_path);
        }
        _ => {
            eprintln!("エラー: 無効なモードです");
            process::exit(1);
        }
    }
}

/**
 * 暗号化処理を実行
 * 
 * @param inputed_data 暗号化対象のデータ
 * @param key_bytes AES-256キー（32バイト）
 * @param file_path 元ファイルパス
 */
fn handle_encryption(inputed_data: &[u8], key_bytes: &[u8; 32], file_path: &str) {
    // デバッグ用：暗号化前のデータを表示
    // TODO: 本番環境では削除またはverboseフラグで制御
    println!("\nOriginal text: {:?}\n", inputed_data);

    // AES-256-CBCによる暗号化処理
    let encrypted = libs::encrypt_aes256(inputed_data, key_bytes);

    // 暗号化結果の出力ファイル生成
    // 元ファイル名に.aes拡張子を付加
    let out_file = format!("{}.aes", file_path);
    let mut file = File::create(&out_file).expect("暗号化ファイルの作成に失敗しました");
    
    file.write_all(&encrypted)
        .expect("暗号化データの書き込みに失敗しました");

    // デバッグ用：暗号化結果をhex形式で表示
    // TODO: 本番環境では削除またはverboseフラグで制御
    println!("\nEncrypted (hex): {}\n", hex::encode(&encrypted));
    println!("暗号化完了: {}", out_file);
}

/**
 * 復号化処理を実行
 * 
 * @param inputed_data 復号化対象のデータ
 * @param key_bytes AES-256キー（32バイト）
 * @param file_path 元ファイルパス
 */
fn handle_decryption(inputed_data: &[u8], key_bytes: &[u8; 32], file_path: &str) {
    // AES-256-CBCによる復号化処理
    let decrypted = libs::decrypt_aes256(inputed_data, key_bytes);

    // デバッグ用：復号結果の表示
    // TODO: 本番環境では削除またはverboseフラグで制御
    println!("\nDecrypted text: {}\n", decrypted);

    // 復号結果の出力ファイル生成
    // 元ファイル名に.dec拡張子を付加
    let out_file = format!("{}.dec", file_path);
    let mut file = File::create(&out_file).expect("復号ファイルの作成に失敗しました");

    // String -> Vec<u8>への変換して書き込み
    // NOTE: バイナリファイルの場合は別途対応が必要
    file.write_all(decrypted.as_bytes())
        .expect("復号データの書き込みに失敗しました");
    
    println!("復号化完了: {}", out_file);
}