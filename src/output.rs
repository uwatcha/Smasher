// output.rs
// 結果表示モジュール
//
// 解析結果を見やすく表示する

use crate::model::{ActionType, AnalysisResult};

/// 解析結果を標準出力に表示する
/// 
/// # 引数
/// * `result` - 解析結果
/// 
/// # 表示内容
/// - 学籍番号
/// - 対戦回次
/// - 各行動タイプの回数
/// - 各行動タイプの比率（%）
/// - 最も多い行動タイプ
pub fn display_result(result: &AnalysisResult) {
    println!("========================================");
    println!("対戦ゲーム行動ログ解析結果");
    println!("========================================");
    println!();
    
    display_player_info(result);
    println!();
    
    display_counts(result);
    println!();

    display_action_id_counts(result);
    println!();

    display_ratios(result);
    println!();
    
    display_most_frequent(result);
    
    println!("========================================");
}

/// プレイヤー情報を表示
fn display_player_info(result: &AnalysisResult) {
    println!("【プレイヤー情報】");
    println!("  学籍番号: {}", result.player_info.student_id);
    println!("  対戦回次: {}", result.player_info.match_number);
}

/// 行動回数を表示
fn display_counts(result: &AnalysisResult) {
    println!("【行動回数】");
    println!("  攻撃   (Attack): {} 回", result.counts.attack_count);
    println!("  シールド(Shield): {} 回", result.counts.shield_count);
    println!("  回避   (Dodge) : {} 回", result.counts.dodge_count);
    println!("  合計           : {} 回", result.counts.total());
}

/// 行動比率を表示
fn display_ratios(result: &AnalysisResult) {
    println!("【行動比率】");
    
    // 小数点以下1桁で表示
    println!("  攻撃   (Attack): {:.1}%", result.counts.attack_ratio());
    println!("  シールド(Shield): {:.1}%", result.counts.shield_ratio());
    println!("  回避   (Dodge) : {:.1}%", result.counts.dodge_ratio());
}

/// 最も多い行動IDを表示（日本語名付き）
fn display_most_frequent(result: &AnalysisResult) {
    println!("【最も多い行動】");
    if let Some((id, count)) = result.action_id_counts.first() {
        let name = ActionType::get_action_name(id);
        println!("  {} ({}) - {}回", name, id, count);
    } else {
        println!("  データがありません");
    }
}

/// 簡易版の結果表示（1行で出力）

// 使われていなくても警告を出さないようにする
#[allow(dead_code)]
pub fn display_result_compact(result: &AnalysisResult) {
    let most = result.action_id_counts.first();
    let summary = if let Some((id, count)) = most {
        let name = ActionType::get_action_name(id);
        format!("{} ({}) - {}回", name, id, count)
    } else {
        "データなし".to_string()
    };

    println!(
        "{} (対戦{}) - Attack:{:.1}%, Shield:{:.1}%, Dodge:{:.1}% → 最多: {}",
        result.player_info.student_id,
        result.player_info.match_number,
        result.counts.attack_ratio(),
        result.counts.shield_ratio(),
        result.counts.dodge_ratio(),
        summary
    );
}

/// 行動IDごとの回数を表示（降順）
fn display_action_id_counts(result: &AnalysisResult) {
    println!("【行動IDごとの回数（降順）】");
    let data = &result.action_id_counts;
    if data.is_empty() {
        println!("  データがありません");
        return;
    }

    let max = data.iter().map(|(_, c)| *c).max().unwrap_or(0);
    if max == 0 {
        println!("  データがありません");
        return;
    }

    const MAX_WIDTH: u32 = 30;
    let to_bar = |count: u32| -> String {
        let width = ((count as f64 / max as f64) * MAX_WIDTH as f64).round() as u32;
        let width = width.max(1);
        "#".repeat(width as usize)
    };

    for (id, count) in data {
        let bar = to_bar(*count);
        println!("  {:<12}: {}", id, bar);
    }
}