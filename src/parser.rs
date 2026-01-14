// parser.rs
// CSV読み込み処理モジュール
//
// CSVファイルをBattleLog構造体に変換する

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::error::{Result, SmasherError};
use crate::model::{Action, BattleLog, PlayerInfo};

/// CSVファイルから対戦ログを読み込む
/// 
/// # ファイル形式
/// ```
/// b1022024,1
/// 1.04,us
/// 1.64,ss
/// 2.41,ds
/// ```
/// 
/// 1行目: 学籍番号,対戦回次
/// 2行目以降: タイムスタンプ,行動ID
/// 
/// # 戻り値
/// 読み込んだBattleLog、またはエラー
/// 
/// # エラー
/// - ファイルが開けない
/// - 形式が不正
/// - データの解析に失敗
pub fn read_battle_log<P: AsRef<Path>>(file_path: P) -> Result<BattleLog> {
    // ファイルを開く
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    
    let mut lines = reader.lines();
    
    // 1行目: プレイヤー情報を読み込む
    let player_info = parse_player_info(&mut lines)?;
    
    // 2行目以降: 行動データを読み込む
    let actions = parse_actions(&mut lines)?;
    
    // 行動データが空でないか確認
    if actions.is_empty() {
        return Err(SmasherError::EmptyData(
            "行動データが1つも見つかりませんでした".to_string()
        ));
    }
    
    // BattleLogを作成して返す
    Ok(BattleLog::new(player_info, actions))
}

/// プレイヤー情報を解析する（1行目）
/// 
/// # 引数
/// * `lines` - ファイルの行イテレータ
/// 
/// # 戻り値
/// 解析されたPlayerInfo、またはエラー
fn parse_player_info<B: BufRead>(lines: &mut std::io::Lines<B>) -> Result<PlayerInfo> {
    // 1行目を読み込む
    let first_line = lines
    // 読む行を1つ進める
        .next()
        .ok_or_else(|| SmasherError::EmptyData("ファイルが空です".to_string()))??;
    
    // カンマで分割
    let parts: Vec<&str> = first_line.split(',').collect();
    
    // 2つの要素（学籍番号、対戦回次）があるか確認
    if parts.len() != 2 {
        return Err(SmasherError::InvalidFormat(
            format!("1行目は「学籍番号,対戦回次」の形式である必要があります。実際: {}", first_line)
        ));
    }
    
    // 学籍番号を取得（文字列のまま）
    let student_id = parts[0].trim().to_string();
    
    // 対戦回次を数値に変換
    let match_number = parts[1]
        .trim()
        .parse::<u32>()
        .map_err(|_| SmasherError::ParseError(
            format!("対戦回次を数値に変換できません: {}", parts[1])
        ))?;
    
    // Javaでいう return。Error時はErr()を使う
    Ok(PlayerInfo::new(student_id, match_number))
}

/// 行動データを解析する（2行目以降）
/// 
/// # 引数
/// * `lines` - ファイルの行イテレータ
/// 
/// # 戻り値
/// 解析されたActionのベクタ、またはエラー
fn parse_actions<B: BufRead>(lines: &mut std::io::Lines<B>) -> Result<Vec<Action>> {
    let mut actions = Vec::new();
    
    // 残りの行を1行ずつ処理
    // parse_player_infoで1行目を読んでいるので、ここでは2行目以降を読む
    for (line_number, line_result) in lines.enumerate() {
        // 行を読み込む（エラーがあれば?で返す）
        let line = line_result?;
        
        // 空行はスキップ
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        // 1つの行動を解析
        let action = parse_action_line(trimmed, line_number + 2)?;
        actions.push(action);
    }
    
    Ok(actions)
}

/// 1行の行動データを解析する
/// 
/// # 引数
/// * `line` - 行の文字列（例: "1.04,us"）
/// * `line_number` - 行番号（エラーメッセージ用）
/// 
/// # 戻り値
/// 解析されたAction、またはエラー
fn parse_action_line(line: &str, line_number: usize) -> Result<Action> {
    let parts: Vec<&str> = line.split(',').collect();
    
    // 2つの要素（タイムスタンプ、行動ID）があるか確認
    if parts.len() != 2 {
        return Err(SmasherError::InvalidFormat(
            format!("{}行目: 「タイムスタンプ,行動ID」の形式である必要があります。実際: {}", line_number, line)
        ));
    }
    
    // タイムスタンプを浮動小数点数に変換
    let timestamp = parts[0]
        .trim()
        .parse::<f64>()
        .map_err(|_| SmasherError::ParseError(
            format!("{}行目: タイムスタンプを数値に変換できません: {}", line_number, parts[0])
        ))?;
    
    // 行動IDを取得
    let action_id = parts[1].trim().to_string();
    
    // Actionを作成
    Ok(Action::new(timestamp, action_id))
}


