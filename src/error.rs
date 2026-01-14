// error.rs
// エラー型定義モジュール
//
// このモジュールでは、プログラム内で発生する可能性のあるエラーを
// 型として定義します。

use std::fmt;
use std::io;

/// プログラム内で発生するエラーを表すenum
/// 
/// Rustでは、エラーの種類をenumで表現することで、
/// どんなエラーが発生したのかを明確に扱える
#[derive(Debug)]
pub enum SmasherError {
    /// ファイル入出力に関するエラー
    /// 
    /// ファイルが見つからない、読み込めないなど
    IoError(io::Error),
    
    /// CSV形式が不正な場合のエラー
    /// 
    /// 例：カンマで区切られていない、必要な列が足りないなど
    InvalidFormat(String),
    
    /// データの解析に失敗した場合のエラー
    /// 
    /// 例：数値に変換できない、タイムスタンプが不正、など
    ParseError(String),
    
    /// データが空の場合のエラー
    EmptyData(String),
}

// Displayトレイト (interfaceのようなもの) を実装することで、エラーメッセージを表示できるようにする
// println!("{}", error) のように使えるようになる
// ↓SmasherError型にfmt::Displayトレイトを実装
impl fmt::Display for SmasherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      // switch文  
      match self {
            SmasherError::IoError(err) => {
                write!(f, "ファイル入出力エラー: {}", err)
            }
            SmasherError::InvalidFormat(msg) => {
                write!(f, "CSV形式エラー: {}", msg)
            }
            SmasherError::ParseError(msg) => {
                write!(f, "データ解析エラー: {}", msg)
            }
            SmasherError::EmptyData(msg) => {
                write!(f, "データが空です: {}", msg)
            }
        }
    }
}

// std::error::Errorトレイトを実装
// これにより、標準のエラー処理の仕組みに乗せることができる
impl std::error::Error for SmasherError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // IoErrorの場合のみ、元のエラーを返す
        match self {
            SmasherError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

// io::ErrorからSmasherErrorへの変換を定義
// これにより、?演算子でio::Errorを自動的にSmasherErrorに変換できる
impl From<io::Error> for SmasherError {
    fn from(err: io::Error) -> Self {
        SmasherError::IoError(err)
    }
}

/// Result型のエイリアス
/// 
/// SmasherErrorを使ったResult型の使用を簡略化するためのエイリアスを定義する
/// 
/// 使用例：
/// ```
/// fn some_function() -> Result<String> {
///     Ok("成功".to_string())
/// }
/// ```
pub type Result<T> = std::result::Result<T, SmasherError>;