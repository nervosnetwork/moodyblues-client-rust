use std::time::{SystemTime, UNIX_EPOCH};

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn now_in_millisec_should_gt_0() {
        assert!(now() > 0);
    }
}
