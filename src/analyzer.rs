// analyzer.rs
// 行動分析モジュール
//
// BattleLogから行動を集計し、統計情報を計算

// 意味：「このプロジェクト内で、定義したmodelの、ActionCounts, AnalysisResult, BattleLogを使いますという宣言」
use crate::model::{ActionCounts, AnalysisResult, BattleLog};

/// # 処理の流れ
/// 1. 各行動タイプの出現回数を数える
/// 2. ActionCountsを作成
/// 3. AnalysisResultを作成（内部で比率計算も行われる）
/// 
/// # 戻り値
/// 解析結果（AnalysisResult）
pub fn analyze(battle_log: &BattleLog) -> AnalysisResult {
    let counts = count_actions(battle_log);
    let action_id_counts = count_actions_by_id(battle_log);
    
    // 解析結果を作成（比率計算も含む）
    AnalysisResult::new(battle_log, counts, action_id_counts)
}

/// 各行動タイプの出現回数を数える
fn count_actions(battle_log: &BattleLog) -> ActionCounts {
    // カウンターを初期化（すべて0）
    let mut counts = ActionCounts::new();
    
    // すべての行動をループで処理
    for action in &battle_log.actions {
        // 行動タイプに応じてカウントを増やす
        counts.increment(&action.action_type);
    }
    
    counts
}

/// 行動IDごとの出現回数を数える（降順ソート）
fn count_actions_by_id(battle_log: &BattleLog) -> Vec<(String, u32)> {
  // BTreeMap: Dictionaryのように使用できる
    let mut map: std::collections::BTreeMap<String, u32> = std::collections::BTreeMap::new();
    for action in &battle_log.actions {
        *map.entry(action.original_id.clone()).or_insert(0) += 1;
    }
    let mut items: Vec<(String, u32)> = map.into_iter().collect();
    // 回数降順、同回数ならID昇順
    items.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    items
}
