#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod engine;
mod logs;
mod portfolio;
mod trader;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //
    logs::logs_guard();

    //
    engine::start_engine();

    Ok(())
}
