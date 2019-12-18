# MoodyBlues SDK For Rust

## Intro

A tracer SDK for [overlord][overlord] like consensus algorithm, helps you to debug 
or optimize the algorithm.

The consensus algorithm always plays with a distributed system, 
debugging or optimizing is so hard. If we can record events to describe what happens 
when consensus state changes, and then replay with a visualization dashboard, 
the debugging or optimizing would be easier.

## Quick start

Let's starts with a dead simple trace. This example shows how to use the sdk
for write the TracePoint in file with JSON format.

Cargo.toml

```toml
[dependencies]
moodyblues-sdk = { git = "https://github.com/nervosnetwork/moodyblues-client-rust" }
```

consensus.rs
```rust
use moodyblues_sdk::trace;

struct ConsensusStateMachine {
  epoch_id: u64,
  round_id: u64
}

impl ConsensusStateMachine {
  fn new_epoch(&mut self, epoch_id: u64) {
    self.epoch_id = self.epoch_id + 1;
    self.round_id = 0;
    // create a trace point mark as starts with epoch
    trace::start_epoch(epoch_id);
  }
  
  fn new_round(&mut self, round_id: u64) {
    self.round_id = self.round;
    
    // create a trace point mark as starts with round
    trace::start_round(round_id);
  }
}


struct Consensus;

impl Consensus {
  fn verify_singature(signature: Signature, hash: Hash) {
    trace::report_custom("verify_singature".to_string(), Some(json!({
      "hash": hash,
      "signature": signature
    })))
  }
}
```





main.rs

```rust
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

use serde_json::{json, to_string};

use moodyblues_sdk::event::{EventType, TraceEvent};
use moodyblues_sdk::point::{Metadata, TracePoint};
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
        // timestamp for each point
        now()
    }
}

fn main() {
  trace::set_boxed_tracer(WriteReporter::new(File::create("log.log").unwrap()));
}
```

## Documentation

TODO for now, jump to `trace.rs` for more information. The API may be frequent changes before the first release version.

[overlord]: https://github.com/nervosnetwork/overlord