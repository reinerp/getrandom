pub(super) fn is_success(status: u32) -> bool {
    status & (1 << 31) == 0
}

#[test]
fn success_excludes_warnings() {
    assert!(is_success(0));
    assert!(is_success(0x4000_0000));
    assert!(!is_success(0x8000_0000));
    assert!(!is_success(0x8000_0005));
    assert!(!is_success(0xc000_0001));
}
