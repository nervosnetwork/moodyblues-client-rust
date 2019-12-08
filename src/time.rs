use std::time::{SystemTime, UNIX_EPOCH};

pub fn now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn now_in_millisec_should_gt_0() {
        assert!(now() > 0);
    }
}
