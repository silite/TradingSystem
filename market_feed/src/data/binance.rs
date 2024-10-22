use std::{
    cell::{Cell, RefCell},
    ops::Deref,
    rc::Rc,
    sync::Arc,
};

use crossbeam::channel::{Receiver, Sender};
use derive_builder::Builder;
use futures::FutureExt;
use protocol::{
    event::{DataKind, MarketEvent},
    indictor::BundleMarketIndicator,
    portfolio::market_data::binance::Kline,
};
use tokio::sync::{mpsc, oneshot};

use crate::{
    indictor::{Indicator, IndicatorsCollection},
    MarketFeed,
};

pub enum BinanceMarketFeedCommand {
    LoadHistory,
}

pub struct BinanceMarketFeed {
    indicators: IndicatorsCollection,
    subscribe_channel: Sender<MarketEvent>,
    command_rx: Option<tokio::sync::mpsc::UnboundedReceiver<BinanceMarketFeedCommand>>,
}

impl Clone for BinanceMarketFeed {
    fn clone(&self) -> Self {
        BinanceMarketFeed {
            indicators: self.indicators.clone(),
            subscribe_channel: self.subscribe_channel.clone(),
            command_rx: None,
        }
    }
}

impl MarketFeed for BinanceMarketFeed {
    type MarketData = Kline;

    type Command = BinanceMarketFeedCommand;

    fn new() -> (
        mpsc::UnboundedSender<Self::Command>,
        crossbeam::channel::Receiver<MarketEvent>,
    ) {
        let (command_tx, command_rx) = mpsc::unbounded_channel();
        let (data_tx, data_rx) = crossbeam::channel::unbounded();
        let inst = Self {
            indicators: IndicatorsCollection::new(),
            subscribe_channel: data_tx,
            command_rx: Some(command_rx),
        };
        let handle = tokio::spawn(async {
            inst.run().await.unwrap();
        });

        (command_tx, data_rx)
    }

    async fn run(mut self) -> anyhow::Result<()> {
        if let Some(mut command_rx) = self.command_rx.take() {
            ftlog::info!("[binance market feed] run.");
            loop {
                tokio::select! {
                    Some(command) = command_rx.recv() => {
                        match command {
                            BinanceMarketFeedCommand::LoadHistory => {
                                let history_cnt = self.load_history_market_data().await?;
                                ftlog::info!("Load market data history done. cnt({})", history_cnt);
                            }
                        };
                    }
                }
            }
        } else {
            // 这里Clone可能有问题，还没检查
            println!("error sender is None!");
        }
        Ok(())
    }

    async fn load_history_market_data(&mut self) -> anyhow::Result<i32> {
        ftlog::info!("[binance market feed] start load_history_market_data.");
        let (tx, rx) = crossbeam::channel::unbounded::<Kline>();

        // 等待spawn成功
        let (start_tx, start_rx) = oneshot::channel();

        let select_join = tokio::spawn(async {
            let _ = start_tx.send(());
            db::market_data::select_all(tx, "btcusdt_kline").await
        });

        start_rx.await?;

        while let Ok(kline) = rx.recv() {
            self.computed_indicator(&kline);
        }

        let data_cnt = select_join.await??;

        Ok(data_cnt)
    }

    async fn handle_market_data(&mut self) -> anyhow::Result<()> {
        todo!()
    }

    fn is_linked(&self) -> bool {
        todo!()
    }

    fn computed_indicator(&mut self, data: &Self::MarketData) {
        let dc = self.indicators.dc.push(data).get();
        let rsi = self.indicators.rsi.push(data).get();
        let ema = self.indicators.ema.push(data).get();
        let stoch_rsi = self.indicators.stock_rsi.push(rsi).get(rsi);
        let adx = self.indicators.adx.push(data).get();
        let macd = self.indicators.macd.push(data).get();
        let tr = self.indicators.tr.push(data).get();
        let tr_rma = self.indicators.tr_rma.push(tr).get();

        let atr_low = data.low - self.indicators.pre_tr_rma * 1.5;
        let atr_high = self.indicators.pre_tr_rma * 1.5 + data.high;
        self.indicators.pre_tr_rma = tr_rma;

        let bundle_data = BundleMarketIndicator {
            market_data: data.clone(),
            dc,
            rsi,
            ema,
            stoch_rsi,
            adx,
            macd,
            tr_rma,
            tr,
            atr: (atr_low, atr_high),
        };

        if let Err(e) = self.subscribe_channel.send(MarketEvent {
            kind: DataKind::BundleData(bundle_data),
        }) {
            ftlog::error!("send computed indicator error {}", e);
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 6)]
async fn test_market_feed() {
    utils::logs::logs_guard();
    let (command_tx, data_rx) = BinanceMarketFeed::new();
    command_tx
        .send(BinanceMarketFeedCommand::LoadHistory)
        .map_err(|e| ftlog::error!("{:?}", e))
        .unwrap_or_default();
    while let Ok(computed_data) = data_rx.recv() {
        println!("{:?}", computed_data);
    }
}
