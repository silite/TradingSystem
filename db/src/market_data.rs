use std::fmt::Debug;

use clickhouse::Row;
use serde::{de::DeserializeOwned, Serialize};

use crate::CLIENT;

pub async fn select_all<T: Row + DeserializeOwned + Debug>(
    sender: crossbeam::channel::Sender<T>,
    table: &str,
) -> anyhow::Result<i32> {
    let mut cursor = CLIENT
        .query(&format!(r#"SELECT * FROM `{}`"#, table))
        .fetch::<T>()?;
    let mut cnt = 0;
    while let Some(row) = cursor.next().await? {
        cnt += 1;
        if let Err(e) = sender.send(row) {
            ftlog::error!("send market_error {:?}", e)
        }
    }
    Ok(cnt)
}

pub async fn batch_insert<T: Row + Serialize>(
    data_list: Vec<T>,
    table: &str,
) -> anyhow::Result<()> {
    let mut insert = CLIENT.insert(table)?;
    for data in data_list {
        insert.write(&data).await?;
    }
    insert.end().await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use protocol::portfolio::market_data::binance::Kline;

    use crate::market_data::batch_insert;

    #[tokio::test]
    async fn test_read_csv() {
        let res = utils::history_data_load::load_csv::<Kline>(
        "/Users/siliterong/Project/rust/TradingSystem/utils/src/history_data_load/binance_kline",
        )
        .unwrap();
        println!("{:?}", res.len());
        batch_insert(res, "btcusdt_kline").await.unwrap();
    }
}
