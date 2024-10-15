pub fn logs_guard() {
    #[allow(unused_variables)]
    let guard = ftlog::builder().build().unwrap().init().unwrap();
}
