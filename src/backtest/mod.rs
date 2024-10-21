use utils::logs;

mod engine;
mod matching;
mod portfolio;
mod trader;

pub async fn start() -> anyhow::Result<()> {
    //
    logs::logs_guard();

    //
    engine::start_engine().await;

    Ok(())
}
