use clap::{Arg, Command};
use std::fs;
use hex;
mod libs;
use std::fs::File;
use std::io::Write;
use std::process;

fn main() {
    let matches = Command::new("cypherAES")
        .about("AES-256-CBC Encrypt/Decrypt CLI Tool")
        .arg(
            Arg::new("mode")
                .short('m')
                .long("mode")
                .help("Mode: encrypt or decrypt")
                .required(true)
                .value_parser(["encrypt", "decrypt"])
                .value_name("MODE")
        )
        .arg(
            Arg::new("keyfile")
                .short('k')
                .long("keyfile")
                .help("Path to the 32-byte key file")
                .required(true)
                .value_name("KEYFILE")
        )
        .arg(
            Arg::new("filepath")
                .short('f')
                .long("filepath")
                .help("file to be encripted")
                .required(true)
                .value_name("FILEPATH")
        )
        .get_matches();

    // モード取得
    let mode = matches.get_one::<String>("mode").expect("モードが取得できません");

    // キーファイル取得
    let keyfile = matches.get_one::<String>("keyfile").expect("キーファイル名が取得できません");
    let key_bytes = fs::read(keyfile).expect("キーファイルの読み込みに失敗しました");
    if key_bytes.len() != 32 {
        eprint!("キーファイルは32バイト(256bit)で指定してください");
        process::exit(1);
    }
    let key_bytes: [u8; 32] = key_bytes.try_into().expect("32バイト配列に変換できません");

    //対象ファイルの読み込み
    let file_path = matches.get_one::<String>("filepath").expect("対象ファイルが読み込めません");
    let inputed_data = fs::read(file_path).expect("Failed to read file");


    // modeによる分岐
    if mode == "encrypt" {
        //TEST　　暗号化前の文字列を表示
        println!("\nOriginal text: {:?}\n", inputed_data);
        
        // 暗号化     
    
        let encrypted = libs::encrypt_aes256(&inputed_data, &key_bytes);
        let out_file =String::from(file_path) + ".aes";
        let mut file = File::create(out_file).expect("Cannot create file");
        file.write_all(&encrypted).expect("Write failed");

        // TEST 暗号化結果を表示
        println!("\nEncrypted (hex): {}\n", hex::encode(&encrypted));
    } else if mode == "decrypt" {
        // 復号化
        let decrypted = libs::decrypt_aes256(&inputed_data, &key_bytes);

        //Test　復号結果の表示
        println!("\nDecrypted text: {}\n", decrypted);
        let out_file =String::from(file_path) + ".dec";
        let mut file = File::create(out_file).expect("復号結果がファイルが作成できません");
        file.write_all(&decrypted.into_bytes()).expect("復号結果がファイルに書き込み出来ません");
    }  
}

