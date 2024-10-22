use std::{
    cell::{Cell, RefCell},
    ops::Deref,
    rc::Rc,
    sync::{Arc, Mutex},
};

use crossbeam::channel::{Receiver, Sender};
use derive_builder::Builder;
use futures::FutureExt;
use protocol::{
    event::{DataKind, Event, EventBus, MarketDataEvent, MarketFeedEvent},
    indictor::BundleMarketIndicator,
    portfolio::market_data::binance::Kline,
};
use tokio::sync::{mpsc, oneshot};

use crate::{
    indictor::{Indicator, IndicatorsCollection},
    MarketFeed,
};

#[derive(Clone)]
pub struct BinanceMarketFeed {
    indicators: IndicatorsCollection,
    event_bus: Arc<EventBus>,
    event_rx: crossbeam::channel::Receiver<Event>,
    market_tx_topic: &'static str,
}

impl MarketFeed for BinanceMarketFeed {
    type MarketData = Kline;

    fn new(event_bus: Arc<EventBus>, market_tx_topic: &'static str) -> Self {
        let event_rx = event_bus.subscribe("binance_market_feed".to_owned());

        Self {
            indicators: IndicatorsCollection::new(),
            event_bus,
            market_tx_topic,
            event_rx,
        }

        // tokio::spawn(async {
        //     inst.run().await?;
        //     Ok(())
        // })
    }

    async fn run(mut self) -> anyhow::Result<()> {
        ftlog::info!("[binance market feed] run.");
        while let Ok(event) = self.event_rx.recv() {
            match event {
                Event::MarketFeed(market_feed_event) => match market_feed_event {
                    MarketFeedEvent::LoadHistory => {
                        let history_cnt = self.load_history_market_data().await?;
                        ftlog::info!("Load market data history done. cnt({})", history_cnt);
                    }
                },
                _ => unreachable!(),
            }
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

        if let Err(e) = self.event_bus.publish(
            self.market_tx_topic,
            Event::MarketData(MarketDataEvent {
                kind: DataKind::BundleData(bundle_data),
            }),
        ) {
            ftlog::error!("send computed indicator error {}", e);
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 6)]
async fn test_market_feed() {
    utils::logs::logs_guard();
    // let (command_tx, data_rx) = BinanceMarketFeed::new();
    // command_tx
    //     .send(BinanceMarketFeedCommand::LoadHistory)
    //     .map_err(|e| ftlog::error!("{:?}", e))
    //     .unwrap_or_default();
    // while let Ok(computed_data) = data_rx.recv() {
    //     println!("{:?}", computed_data);
    // }
}
