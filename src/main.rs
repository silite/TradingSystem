#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod backtest;
mod live;

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() -> anyhow::Result<()> {
    backtest::start().await
}
