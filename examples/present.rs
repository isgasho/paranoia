fn main() {
    if true {
        paranoia_caller::mark();
    }
    assert!(paranoia::marker_exists());
}
