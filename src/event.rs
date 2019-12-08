use serde::{Deserialize, Serialize};

use serde_json::Value;

pub type Json = Value;

/// 标记事件的类型
#[derive(Debug, Serialize, Deserialize)]
pub enum EventType {
    /// 提案事件，可用于标记发起提案或者收到提案
    Propose { proposer: String, hash: String },
    /// 投票事件，可用于标记投票或者收到投票
    Vote { voter: String, hash: String },
    /// 错误事件
    Error,
    /// 关键事件，在一个 round 中，keyframe 标志着一个 step 的开始
    Keyframe { frame_info: Keyframe },
    /// 自定义事件
    Custom,
}

/// 标识关键事件
#[derive(Debug, Serialize, Deserialize)]
pub enum Keyframe {
    /// 当开始新的 Epoch 时的事件标识
    NewEpoch { epoch_id: u64 },
    /// 当开始新一轮投票时的事件标识
    NewRound { round_id: u64 },
    /// 当开始新一个 Step 时的事件表标识
    NewStep { step_name: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraceEvent {
    /// 事件名，用于标识事件，需保证不同事件有不同名称
    pub event_name: String,
    /// 事件的类型
    pub event_type: EventType,

    /// 额外关心的指标，使用 json 格式表示
    /// 注意：tag 中有预留的关键key
    /// error: bool 默认false，表示是否为错误事件
    /// log.message: Option<String>  日志信息
    /// log.stack: Option<String> 默认为空 堆栈信息
    pub tag: Option<Json>,
}
