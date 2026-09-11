pub(super) fn is_ready(count: i32, events: i16) -> bool {
    // Only POLLIN (0x0001 on Linux/Android) was requested. Error/hangup/invalid
    // descriptor events are not evidence that the entropy pool is ready.
    count == 1 && events == 0x0001
}

#[test]
fn readiness_requires_readable_event() {
    assert!(is_ready(1, 1));
    for (count, events) in [(0, 0), (1, 0), (1, 8), (1, 16), (1, 32)] {
        assert!(!is_ready(count, events));
    }
}
