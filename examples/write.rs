use std::fs::File;
use std::io::{prelude::*, BufReader, LineWriter, Write};
use std::sync::Mutex;

use serde_json::{json, to_string};

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
        file.write_all(to_string(&point).unwrap().as_bytes())
            .expect("write file failed");
        file.write_all(b"\n").expect("write file failed");
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

fn main() -> std::io::Result<()> {
    trace::set_boxed_tracer(WriteReporter::new(LineWriter::new(
        File::create("examples/write.log").unwrap(),
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

    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}
