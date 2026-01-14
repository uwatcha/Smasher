// model.rs
// データ構造定義モジュール
//

/// 攻撃系の行動ID一覧
pub const ATTACK_IDS: &[(&str, &str)] = &[
    // 弱攻撃
    ("j1", "弱1段"),
    ("j2", "弱2段"),
    // 強攻撃
    ("st", "横強"),
    ("ut", "上強"),
    ("dt", "下強"),
    ("DA", "ダッシュアタック"),
    // スマッシュ
    ("ss", "横スマ"),
    ("us", "上スマ"),
    ("ds", "下スマ"),
    // 空中攻撃
    ("na", "空N"),
    ("fa", "空前"),
    ("ba", "空後"),
    ("ua", "空上"),
    ("da", "空下"),
    // 必殺技
    ("nb_c", "NB（タメ）"),
    ("nb_a", "NB（攻撃）"),
    ("sb", "横B"),
    ("ub_g", "上B（地上）"),
    ("ub_a", "上B（空中）"),
    ("db_g", "下B（地上）"),
    ("db_a", "下B（空中）"),
    // つかみ
    ("g", "つかみ"),
    ("ga", "つかみ攻撃"),
    ("fth", "前投げ"),
    ("bth", "後投げ"),
    ("uth", "上投げ"),
    ("dth", "下投げ"),
    ("fc", "前投げ（前派生）"),
    ("bc", "前投げ（後派生）"),
    ("uc", "前投げ（上派生）"),
    ("dc", "前投げ（下派生）"),
];

/// シールド系の行動ID一覧
pub const SHIELD_IDS: &[(&str, &str)] = &[
    ("s", "シールド"),
];

/// 回避系の行動ID一覧
pub const DODGE_IDS: &[(&str, &str)] = &[
    ("nd", "その場回避"),
    ("sd", "横回避"),
    ("ad", "空中回避"),
];

#[derive(Debug, Clone, PartialEq, Eq)]
// Debug: デバッグ出力できるようにする
// Clone: 値渡しできるようにする
// PartialEq: == で比較できるようにする
// Eq: JSでいう === を可能にする
pub enum ActionType {
    Attack,
    Shield,
    Dodge,
}

// impl: 型の機能を実装する
impl ActionType {
    pub fn from_action_id(action_id: &str) -> Self {
        // action_idの文字列パターンで分類
        match action_id {
            // シールド系
            "s" => ActionType::Shield,
            
            // 回避系
            "nd" | "sd" | "ad" => ActionType::Dodge,
            
            // 弱攻撃
            "j1" | "j2" => ActionType::Attack,
            
            // 強攻撃
            "st" | "ut" | "dt" | "DA" => ActionType::Attack,
            
            // スマッシュ攻撃
            "ss" | "us" | "ds" => ActionType::Attack,
            
            // 空中攻撃
            "na" | "fa" | "ba" | "ua" | "da" => ActionType::Attack,
            
            // 必殺技
            "nb_c" | "nb_a" | "sb" | "ub_g" | "ub_a" | "db_g" | "db_a" => ActionType::Attack,
            
            // つかみ・投げ
            "g" | "ga" | "fth" | "bth" | "uth" | "dth" | "fc" | "bc" | "uc" | "dc" => ActionType::Attack,
            
            // 上記以外は攻撃として扱う（互換性のため）
            _ => ActionType::Attack,
        }
    }
    
    pub fn get_action_name(action_id: &str) -> String {
        // 攻撃系をチェック
        for (id, name) in ATTACK_IDS {
            if *id == action_id {
                return name.to_string();
            }
        }
        
        // シールド系をチェック
        for (id, name) in SHIELD_IDS {
            if *id == action_id {
                return name.to_string();
            }
        }
        
        // 回避系をチェック
        for (id, name) in DODGE_IDS {
            if *id == action_id {
                return name.to_string();
            }
        }
        
        // 見つからない場合は元のIDを返す
        action_id.to_string()
    }
}

/// 1回の行動を表す
#[derive(Debug, Clone)]
pub struct Action {
    /// 行動が発生した時刻（秒）
    pub timestamp: f64,
    /// 行動の種類
    pub action_type: ActionType,
    /// 元の行動ID
    pub original_id: String,
}

impl Action {
    pub fn new(timestamp: f64, action_id: String) -> Self {
        let action_type = ActionType::from_action_id(&action_id);
        Action {
            timestamp,
            action_type,
            original_id: action_id,
        }
    }
}

/// プレイヤーの情報を表す構造体
#[derive(Debug, Clone)]
pub struct PlayerInfo {
    /// 学籍番号
    pub student_id: String,
    /// 対戦回次
    pub match_number: u32,
}

impl PlayerInfo {
    pub fn new(student_id: String, match_number: u32) -> Self {
        PlayerInfo {
            student_id,
            match_number,
        }
    }
}

/// 1回の対戦における全行動ログ
/// プレイヤー情報と、その対戦中の全行動を保持

#[derive(Debug, Clone)]
pub struct BattleLog {
    pub player_info: PlayerInfo,
    // Vec: 可変長配列
    pub actions: Vec<Action>,
}

impl BattleLog {
    pub fn new(player_info: PlayerInfo, actions: Vec<Action>) -> Self {
        BattleLog {
            player_info,
            actions,
        }
    }
}

/// 各行動タイプの集計結果
/// 各行動タイプの出現回数を保持

#[derive(Debug, Clone)]
pub struct ActionCounts {
    /// 攻撃の回数
    pub attack_count: u32,
    /// シールドの回数
    pub shield_count: u32,
    /// 回避の回数
    pub dodge_count: u32,
}

impl ActionCounts {
    /// すべてのカウントを0で初期化
    pub fn new() -> Self {
        ActionCounts {
            attack_count: 0,
            shield_count: 0,
            dodge_count: 0,
        }
    }

    pub fn total(&self) -> u32 {
        self.attack_count + self.shield_count + self.dodge_count
    }

    // mutをつけないと、定数になる
    pub fn increment(&mut self, action_type: &ActionType) {
        match action_type {
            ActionType::Attack => self.attack_count += 1,
            ActionType::Shield => self.shield_count += 1,
            ActionType::Dodge => self.dodge_count += 1,
        }
    }

    /// 攻撃の比率（%）を計算
    pub fn attack_ratio(&self) -> f64 {
        let total = self.total();
        if total == 0 {
            0.0
        } else {
            (self.attack_count as f64 / total as f64) * 100.0
        }
    }

    /// シールドの比率（%）を計算
    pub fn shield_ratio(&self) -> f64 {
        let total = self.total();
        if total == 0 {
            0.0
        } else {
            (self.shield_count as f64 / total as f64) * 100.0
        }
    }

    /// 回避の比率（%）を計算
    pub fn dodge_ratio(&self) -> f64 {
        let total = self.total();
        if total == 0 {
            0.0
        } else {
            (self.dodge_count as f64 / total as f64) * 100.0
        }
    }

    /// 最も比率が高い行動タイプを返す
    pub fn most_frequent_action(&self) -> ActionType {
        let attack = self.attack_ratio();
        let shield = self.shield_ratio();
        let dodge = self.dodge_ratio();
        
        if attack >= shield && attack >= dodge {
            ActionType::Attack
        } else if shield >= dodge {
            ActionType::Shield
        } else {
            ActionType::Dodge
        }
    }
}


/// プレイヤー情報とカウント情報からなる解析結果
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    /// プレイヤー情報
    pub player_info: PlayerInfo,
    /// 各行動の回数
    pub counts: ActionCounts,
    /// 行動IDごとの回数（降順ソート済み）
    pub action_id_counts: Vec<(String, u32)>,
}

impl AnalysisResult {
    /// BattleLogとActionCountsから解析結果を構築
    /// 
    /// # 引数
    /// * `battle_log` - 対戦ログデータ
    /// * `counts` - 集計済みの行動回数
    /// * `action_id_counts` - 行動IDごとの回数
    /// 
    /// # 戻り値
    /// 解析結果
    pub fn new(battle_log: &BattleLog, counts: ActionCounts, action_id_counts: Vec<(String, u32)>) -> Self {
        AnalysisResult {
            player_info: battle_log.player_info.clone(),
            counts,
            action_id_counts,
        }
    }
}