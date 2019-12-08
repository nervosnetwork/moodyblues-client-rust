use serde::{Deserialize, Serialize};

use crate::event::{EventType, Json, Keyframe, TraceEvent};
use crate::time::now;

static mut TRACER: &'static dyn Trace = &PrintTrace;

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TracePoint {
    timestamp: u128,
    event: TraceEvent,
    metadata: Metadata,
}

pub trait Trace: Sync + Send {
    fn report(&self, event: TracePoint);
    fn metadata(&self) -> Metadata;
    fn now(&self) -> u128;
}

struct PrintTrace;

impl Trace for PrintTrace {
    fn report(&self, point: TracePoint) {
        println!("{:?}", point);
    }

    fn metadata(&self) -> Metadata {
        Metadata {
            address: "".to_string(),
        }
    }

    fn now(&self) -> u128 {
        now()
    }
}

fn tracer() -> &'static dyn Trace {
    unsafe { TRACER }
}

#[derive(Debug)]
pub struct SetTraceError;

pub fn set_boxed_tracer(tracer: Box<dyn Trace>) -> Result<(), SetTraceError> {
    set_tracer(|| unsafe { &*Box::into_raw(tracer) });
    Ok(())
}

fn set_tracer<F>(make_tracer: F) -> Result<(), SetTraceError>
where
    F: FnOnce() -> &'static dyn Trace,
{
    unsafe {
        TRACER = make_tracer();
        Ok(())
    }
}

fn report(event: TraceEvent) {
    let t = tracer();
    let metadata = t.metadata();
    t.report(TracePoint {
        timestamp: t.now(),
        event,
        metadata,
    });
}

fn report_keyframe(event_name: String, keyframe: Keyframe) {
    report(TraceEvent {
        event_name,
        event_type: EventType::Keyframe {
            frame_info: keyframe,
        },
        tag: None,
    })
}

pub fn start_epoch(epoch_id: u64) {
    report_keyframe("start_epoch".to_string(), Keyframe::NewEpoch { epoch_id });
}

pub fn start_round(round_id: u64) {
    report_keyframe("start_round".to_string(), Keyframe::NewRound { round_id })
}

pub fn start_step(step_name: String) {
    report_keyframe("start_step".to_string(), Keyframe::NewStep { step_name })
}

pub fn receive_proposal(event_name: String, proposer: String, hash: String, tag: Option<Json>) {
    report(TraceEvent {
        event_name,
        event_type: EventType::Propose { proposer, hash },
        tag,
    });
}

pub fn receive_vote(event_name: String, voter: String, hash: String, tag: Option<Json>) {
    report(TraceEvent {
        event_name,
        event_type: EventType::Vote { voter, hash },
        tag,
    })
}

pub fn report_custom(event_name: String, tag: Option<Json>) {
    report(TraceEvent {
        event_name,
        event_type: EventType::Custom,
        tag,
    });
}

pub fn report_error(event_name: String, tag: Option<Json>) {
    report(TraceEvent {
        event_name,
        event_type: EventType::Error,
        tag,
    });
}
