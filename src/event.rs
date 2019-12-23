use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type Json = Value;

/// Event type definition
#[derive(Debug, Serialize, Deserialize)]
pub enum EventType {
    /// mark the event as a propose event
    Propose {
        epoch_id: u64,
        round_id: u64,
        proposer: String,
        hash: String,
    },
    /// mark as a vote event
    Vote {
        epoch_id: u64,
        round_id: u64,
        voter: String,
        hash: String,
    },
    /// mark as an error event
    Error,
    /// mark the event as a keyframe
    Keyframe { frame_info: Keyframe },
    /// mark the event as a custom event
    Custom,
}

/// A keyframe is a start point of a phase, the Tendermint-like algorithm always
/// through a series of steps to reach a consensus like this
/// ```graph
/// +-----------------------------------------------+
/// |                 epoch 1                       |
/// +--+---------------------------------------+----+
///    |              round 1                  |
///    +-+-----------+------------+----------+-+
///      |   step 1  |   step 2   |  step .. |
///      +-----------+------------+----------+
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub enum Keyframe {
    /// mark the event as starts of epoch
    NewEpoch { epoch_id: u64 },
    /// mark the event as starts of round
    NewRound { round_id: u64, epoch_id: u64 },
    /// mark the event as starts of step
    NewStep {
        step_name: String,
        round_id: u64,
        epoch_id: u64,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraceEvent {
    /// event name
    pub event_name: String,
    /// event type
    pub event_type: EventType,
    /// metrics of the event, for example, I want to check the order of transactions is correct, and
    /// I want to show what params in the request, so emit an event like this
    /// ```json
    /// {
    ///   "event_name" : "request_check_transaction_order",
    ///   "event_type" : "custom",
    ///   "tag": {
    ///     "epoch_id": 1,
    ///     "transactions": ["0x...", "0x..."]
    ///   }
    /// }
    /// ```
    pub tag: Option<Json>,
}
