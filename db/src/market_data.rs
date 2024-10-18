use clickhouse::Row;
use serde::de::DeserializeOwned;

use crate::CLIENT;

pub async fn select_all<T: Row + DeserializeOwned>(
    sender: crossbeam::channel::Sender<T>,
) -> anyhow::Result<()> {
    let mut cursor = CLIENT.query(r#"SELECT * FROM BTCUSDT"#).fetch::<T>()?;
    while let Some(row) = cursor.next().await? {
        if let Err(e) = sender.send(row) {
            ftlog::error!("send market_error {:?}", e)
        }
    }
    Ok(())
}
