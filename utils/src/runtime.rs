use std::sync::LazyLock;

use tokio::runtime::Runtime;

pub static TOKIO_RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .thread_name("TS-rt")
        .worker_threads(8)
        .enable_all()
        .build()
        .unwrap()
});
