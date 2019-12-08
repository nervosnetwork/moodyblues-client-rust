pub mod event;
pub mod time;
pub mod trace;

#[cfg(test)]
mod test {
    use std::fs::{read_to_string, File};
    use std::io::Write;

    use super::trace;
    use crate::trace::{Metadata, TracePoint};
    use serde_json::to_string;
    use std::sync::Mutex;

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
            file.write_all(to_string(&point).unwrap().as_bytes());
        }

        fn metadata(&self) -> Metadata {
            Metadata {
                address: "0x0000000000000000000000000000000000000000".to_string(),
            }
        }

        fn now(&self) -> u128 {
            0
        }
    }

    #[test]
    fn test_tracer() {
        trace::set_boxed_tracer(WriteReporter::new(File::create("log.log").unwrap()));
        trace::start_epoch(1);

        assert_eq!(read_to_string("log.log").unwrap(), "{\"timestamp\":0,\"event\":{\"event_name\":\"start_epoch\",\"event_type\":{\"Keyframe\":{\"frame_info\":{\"NewEpoch\":{\"epoch_id\":1}}}},\"tag\":null},\"metadata\":{\"address\":\"0x0000000000000000000000000000000000000000\"}}");
    }
}
