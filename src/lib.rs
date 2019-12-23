pub mod event;
pub mod point;
pub mod time;
pub mod trace;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::{prelude::*, BufReader, LineWriter, Write};
    use std::sync::Mutex;

    use serde_json::{from_str, json, to_string, Value};

    use super::event::{EventType, Keyframe, TraceEvent};
    use super::point::{Metadata, TracePoint};
    use super::trace;
    use super::VERSION;

    struct WriteReporter<W: Write + Send + 'static> {
        reporter: Mutex<W>,
    }

    impl<W: Write + Send + 'static> WriteReporter<W> {
        fn new(writable: W) -> Box<WriteReporter<W>> {
            Box::new(WriteReporter {
                reporter: Mutex::new(writable),
            })
        }
    }

    impl<W: Write + Send + 'static> trace::Trace for WriteReporter<W> {
        fn report(&self, point: TracePoint) {
            let mut file = self.reporter.lock().unwrap();
            file.write_all(to_string(&point).unwrap().as_bytes())
                .expect("write file failed");
            file.write_all(b"\n").expect("write file failed");
        }

        fn metadata(&self) -> Metadata {
            Metadata {
                address: "0x0000000000000000000000000000000000000000".to_string(),
            }
        }

        fn now(&self) -> u64 {
            0
        }
    }

    #[test]
    fn test_tracer() -> std::io::Result<()> {
        trace::set_boxed_tracer(WriteReporter::new(LineWriter::new(
            File::create("log.log").unwrap(),
        )))
        .expect("init tracer failed");
        trace::start_epoch(1);
        trace::start_round(0, 1);
        trace::custom(
            "broadcast_proposal".to_string(),
            Some(json!({
              "hash": "0x00",
              "epoch_id": 1,
              "round_id": 0
            })),
        );
        trace::receive_proposal(
            "receive_propose".to_string(),
            1,
            0,
            "0x".to_string(),
            "".to_string(),
            None,
        );
        trace::start_step("propose".to_string(), 0, 1);
        trace::error("check_failed".to_string(), None);

        let file = File::open("log.log")?;
        let reader = BufReader::new(file);
        let events = vec![
            json!({
              "timestamp": 0,
              "event_name": "start_epoch",
              "event_type": "keyframe",
              "tag": {
                "epoch_id": 1
              },
              "metadata": {
                "address": "0x0000000000000000000000000000000000000000",
                "v": VERSION
              }
            }),
            json!({
              "timestamp": 0,
              "event_name": "start_round",
              "event_type": "keyframe",
              "tag": {
                "epoch_id": 1,
                "round_id": 0
              },
              "metadata": {
                "address": "0x0000000000000000000000000000000000000000",
                "v": VERSION
              }
            }),
            json!({
              "timestamp": 0,
              "event_name": "broadcast_proposal",
              "event_type": "custom",
              "tag": {
                "epoch_id": 1,
                "hash": "0x00",
                "round_id": 0
              },
              "metadata": {
                "address": "0x0000000000000000000000000000000000000000",
                "v": VERSION
              }
            }),
            json!({
              "timestamp": 0,
              "event_name": "receive_propose",
              "event_type": "propose",
              "tag": {
                "epoch_id": 1,
                "hash": "",
                "proposer": "0x",
                "round_id": 0
              },
              "metadata": {
                "address": "0x0000000000000000000000000000000000000000",
                "v": VERSION
              }
            }),
            json!({
              "timestamp": 0,
              "event_name": "start_step",
              "event_type": "keyframe",
              "tag": {
                "epoch_id": 1,
                "round_id": 0,
                "step_name": "propose"
              },
              "metadata": {
                "address": "0x0000000000000000000000000000000000000000",
                "v": VERSION
              }
            }),
            json!({
              "timestamp": 0,
              "event_name": "check_failed",
              "event_type": "error",
              "tag": {},
              "metadata": {
                "address": "0x0000000000000000000000000000000000000000",
                "v": VERSION
              }
            }),
        ];

        for (_i, (line, json)) in reader.lines().zip(events.iter()).enumerate() {
            let e: Value = from_str(&line.unwrap()).unwrap();
            let a: Value = from_str(&to_string(json).unwrap()).unwrap();
            assert_eq!(e, a);
        }
        Ok(())
    }
}
