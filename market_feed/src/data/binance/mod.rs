use std::{
    cell::{Cell, RefCell},
    ops::Deref,
    rc::Rc,
    sync::Arc,
};

use crossbeam::channel::{Receiver, Sender};
use derive_builder::Builder;
use futures::FutureExt;
use protocol::portfolio::market_data::binance::Kline;
use tokio::sync::{mpsc, oneshot};

use crate::{
    indictor::{BundleMarketIndicator, Indicator, IndicatorsCollection},
    MarketFeed,
};

pub enum BinanceMarketFeedCommand<T> {
    Subscribe(crossbeam::channel::Sender<T>),
    LoadHistory,
}

pub struct BinanceMarketFeed {
    indicators: IndicatorsCollection,
    subscribe_channel: Option<Sender<BundleMarketIndicator<Kline>>>,
    command_rx: Option<
        tokio::sync::mpsc::UnboundedReceiver<
            BinanceMarketFeedCommand<BundleMarketIndicator<Kline>>,
        >,
    >,
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

    // TODO + ?
    type BundleData = BundleMarketIndicator<Kline>;

    type Command = BinanceMarketFeedCommand<Self::BundleData>;

    fn new() -> (Self, mpsc::UnboundedSender<Self::Command>) {
        let (command_tx, command_rx) = mpsc::unbounded_channel();
        (
            Self {
                indicators: IndicatorsCollection::new(),
                subscribe_channel: None,
                command_rx: Some(command_rx),
            },
            command_tx,
        )
    }

    async fn run(mut self) -> anyhow::Result<()> {
        if let Some(mut command_rx) = self.command_rx.take() {
            ftlog::info!("[binance market feed] run.");
            loop {
                tokio::select! {
                    Some(command) = command_rx.recv() => {
                        match command {
                            BinanceMarketFeedCommand::Subscribe(sender) => {
                                self.subscribe(sender)?;
                            }
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

        if let Some(sender) = &self.subscribe_channel {
            if let Err(e) = sender.send(BundleMarketIndicator {
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
            }) {
                ftlog::error!("send computed indicator error {}", e);
            }
        }
    }

    fn subscribe(&mut self, sender: Sender<BundleMarketIndicator<Kline>>) -> anyhow::Result<()> {
        self.subscribe_channel = Some(sender);
        Ok(())
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 6)]
async fn test_market_feed() {
    utils::logs::logs_guard();
    let (inst, command_tx) = BinanceMarketFeed::new();
    let (tx, rx) = crossbeam::channel::unbounded();
    let handle = tokio::spawn(async {
        inst.run().await.unwrap();
    });
    command_tx
        .send(BinanceMarketFeedCommand::Subscribe(tx))
        .map_err(|e| ftlog::error!("{:?}", e))
        .unwrap_or_default();
    command_tx
        .send(BinanceMarketFeedCommand::LoadHistory)
        .map_err(|e| ftlog::error!("{:?}", e))
        .unwrap_or_default();
    while let Ok(computed_data) = rx.recv() {
        println!("{:?}", computed_data);
    }
}
