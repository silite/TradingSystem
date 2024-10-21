use ftlog::{
    appender::{self, FileAppender},
    Builder,
};

pub fn logs_guard() {
    #[allow(unused_variables)]
    let inst = Builder::new()
        .root(std::io::stdout())
        .root(FileAppender::rotate_with_expire(
            "logs/trading.log",
            appender::Period::Day,
            appender::Duration::days(60),
        ))
        .build()
        .expect("logger build failed")
        .init()
        .expect("set logger failed");
}
