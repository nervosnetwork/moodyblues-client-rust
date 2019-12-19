use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

use serde_json::to_string;

use moodyblues_sdk::point::{Metadata, TracePoint};
use moodyblues_sdk::time::now;
use moodyblues_sdk::trace;

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
        // information of current node
        Metadata {
            address: "0x0000000000000000000000000000000000000000".to_string(),
        }
    }

    fn now(&self) -> u64 {
        now()
    }
}

fn main() {
    trace::set_boxed_tracer(WriteReporter::new(
        File::create("examples/write.log").unwrap(),
    ));
    trace::start_epoch(1);
}
