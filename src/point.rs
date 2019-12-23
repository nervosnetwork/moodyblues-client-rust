use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde_json::{json, Map};

use crate::event::{EventType, Keyframe, TraceEvent};
use crate::VERSION;

#[derive(Debug)]
pub struct Metadata {
    pub address: String,
}

impl Default for Metadata {
    fn default() -> Self {
        Metadata {
            address: "".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct TracePoint {
    pub timestamp: u64,
    pub event: TraceEvent,
    pub metadata: Metadata,
}

/// assign json
/// ```
/// use serde_json::Map;
///
/// let mut map = Map::new();
/// let value1 = 1;
/// let value2 = 2;
/// assign!(map, value1, value2); // { "value1": 1, "value2": 2 }
/// ```
macro_rules! assign {
    ($map:expr) => {};
    ($map:expr, $($key:ident),*) => {
      $(
         $map.insert(String::from(stringify!($key)), json!($key));
      )*
    };
}

impl Serialize for TracePoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let event_name = &self.event.event_name;

        let mut state = serializer.serialize_struct("TracePoint", 2)?;
        state.serialize_field("timestamp", &self.timestamp)?;
        state.serialize_field("event_name", event_name)?;

        let mut tag = match &self.event.tag {
            Some(json) => json.clone(),
            _ => json!({}),
        };

        let map = tag.as_object_mut().unwrap();

        // fill tag and event_type
        match &self.event.event_type {
            EventType::Propose {
                epoch_id,
                round_id,
                proposer,
                hash,
            } => {
                state.serialize_field("event_type", "propose")?;
                assign!(map, epoch_id, round_id, proposer, hash);
            }

            EventType::Vote {
                epoch_id,
                round_id,
                voter,
                hash,
            } => {
                state.serialize_field("event_type", "vote")?;
                assign!(map, epoch_id, round_id, voter, hash);
            }

            EventType::Keyframe { frame_info } => {
                state.serialize_field("event_type", "keyframe")?;

                match frame_info {
                    Keyframe::NewEpoch { epoch_id } => {
                        assign!(map, epoch_id);
                    }
                    Keyframe::NewRound { round_id, epoch_id } => {
                        assign!(map, round_id, epoch_id);
                    }
                    Keyframe::NewStep {
                        step_name,
                        round_id,
                        epoch_id,
                    } => {
                        assign!(map, step_name, round_id, epoch_id);
                    }
                }
            }
            EventType::Error => {
                state.serialize_field("event_type", "error")?;
            }
            _ => {
                state.serialize_field("event_type", "custom")?;
            }
        }

        state.serialize_field("tag", &tag)?;
        let mut metadata = Map::new();
        let address = &self.metadata.address.clone();
        let v = VERSION;
        assign!(metadata, address, v);
        state.serialize_field("metadata", &metadata)?;
        state.end()
    }
}
