pub mod event;
pub mod point;
pub mod time;
pub mod trace;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::Write;
    use std::sync::Mutex;

    use serde_json::{from_str, json, to_string, Value};

    use super::event::{EventType, TraceEvent};
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
                .unwrap();
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
    fn test_tracer() {
        trace::set_boxed_tracer(WriteReporter::new(File::create("log.log").unwrap()));
        let point = TracePoint {
            timestamp: 0,
            event: TraceEvent {
                event_name: "start_epoch".to_string(),
                event_type: EventType::Custom,
                tag: Some(json!({
                  "x": "y"
                })),
            },
            metadata: Metadata {
                address: "".to_string(),
            },
        };
        println!("{:?}", to_string(&point).unwrap());
    }

    #[test]
    fn test_serialize() {
        let point = TracePoint {
            timestamp: 0,
            event: TraceEvent {
                event_name: "receive_propose".to_string(),
                event_type: EventType::Propose {
                    epoch_id: 1,
                    round_id: 0,
                    proposer: "0x10".to_string(),
                    hash: "0x10".to_string(),
                },
                tag: None,
            },
            metadata: Metadata {
                address: "".to_string(),
            },
        };
        let expect = json!({
          "timestamp": 0,
          "event_name": "receive_propose",
          "event_type": "propose",
          "tag": {
            "epoch_id": 1,
            "hash": "0x10",
            "proposer": "0x10",
            "round_id": 0
          },
          "metadata": {
            "address": "",
            "v": VERSION
          }
        });

        let e: Value = from_str(&to_string(&expect).unwrap()).unwrap();
        let a: Value = from_str(&to_string(&point).unwrap()).unwrap();
        assert_eq!(e, a);
    }
}
