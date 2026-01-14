// b1022024 井田和樹


// main.rs
// エントリポイント

// モジュールの宣言
mod model;
mod error;
mod parser;
mod analyzer;
mod output;

use std::env;
use error::Result;

/// # 処理の流れ
/// 1. コマンドライン引数からファイルパスを取得
/// 2. CSVファイルを読み込む
/// 3. データを解析する
/// 4. 結果を表示する
fn main() {
    // プログラムを実行して、エラーが発生したら表示する
    if let Err(e) = run() {
        // エラーメッセージを赤文字で表示（ANSIエスケープシーケンス）
        eprintln!("\x1b[31mエラーが発生しました: {}\x1b[0m", e);
        
        // エラー終了（終了コード1）
        std::process::exit(1);
    }
}

/// Result型を返すことで、エラーハンドリングを?演算子で書ける
/// 
/// # 戻り値
/// 成功時はOk(()), エラー時はErr(SmasherError)
fn run() -> Result<()> {
    // コマンドライン引数を取得
    // ::  Javaでいう . 
    let args: Vec<String> = env::args().collect();
    
    // 引数の数をチェック
    if args.len() < 2 {
        return Err(error::SmasherError::InvalidFormat(
            "ファイルパスが指定されていません".to_string()
        ));
    }
    
    // ファイルパスを取得
    let file_path = &args[1];
    
    // 処理開始メッセージ
    println!("対戦ゲーム行動ログ解析ツール");
    println!("ファイル: {}", file_path);
    println!();
    
    // 1. CSVファイル読み込み
    println!("CSVファイルを読み込んでいます...");
    let battle_log = parser::read_battle_log(file_path)?;
    println!("✓ 読み込み完了: {} 件の行動データ", battle_log.actions.len());
    println!();
    
    // 2. データ解析
    println!("データを解析しています...");
    let result = analyzer::analyze(&battle_log);
    println!("✓ 解析完了");
    println!();
    
    // 3. 結果表示
    output::display_result(&result);
    
    Ok(())
}

