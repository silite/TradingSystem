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
    indictor::Indicators,
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
    market_tx_topic: &'static str,
    event_topic: &'static str,
}

impl MarketFeed for BinanceMarketFeed {
    type MarketData = Kline;

    fn new(
        event_bus: Arc<EventBus>,
        market_tx_topic: &'static str,
        event_topic: &'static str,
    ) -> Self {
        Self {
            indicators: IndicatorsCollection::new(),
            event_bus,
            market_tx_topic,
            event_topic,
        }
    }

    async fn run(mut self) -> anyhow::Result<()> {
        ftlog::info!("[binance market feed] run.");
        let mut event_rx = self.event_bus.subscribe_sync(self.event_topic.to_owned());
        while let Some(event) = event_rx.recv().await {
            match event {
                Event::MarketFeed(market_feed_event) => match market_feed_event {
                    MarketFeedEvent::LoadHistory => match self.load_history_market_data().await {
                        Ok(history_cnt) => {
                            ftlog::info!("Load market data history done. cnt({})", history_cnt);
                        }
                        Err(err) => {
                            ftlog::error!("[market feed] load_history_market_data error. {:?}", err)
                        }
                    },
                },
                _ => unreachable!(),
            }
        }
        Ok(())
    }

    async fn load_history_market_data(&mut self) -> anyhow::Result<i32> {
        // 这里也可以改总线，发送全局Kline
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
            self.computed_indicator(kline);
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

    fn computed_indicator(&mut self, market_data: Self::MarketData) {
        let dc = self.indicators.dc.push(&market_data).get();
        let rsi = self.indicators.rsi.push(&market_data).get();
        let ema = self.indicators.ema.push(&market_data).get();
        let stoch_rsi = self.indicators.stock_rsi.push(rsi).get(rsi);
        let adx = self.indicators.adx.push(&market_data).get();
        let macd = self.indicators.macd.push(&market_data).get();
        let tr = self.indicators.tr.push(&market_data).get();
        let tr_rma = self.indicators.tr_rma.push(tr).get();

        let atr_low = market_data.low - self.indicators.pre_tr_rma * 1.5;
        let atr_high = self.indicators.pre_tr_rma * 1.5 + market_data.high;
        self.indicators.pre_tr_rma = tr_rma;

        let indicators = Indicators {
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
                kind: DataKind::BundleData((market_data, indicators)),
            }),
        ) {
            ftlog::error!(
                "send computed indicator error {}. topic: {}",
                e,
                self.market_tx_topic
            );
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 6)]
async fn test_market_feed() {
    utils::logs::logs_guard();
    let event_bus = Arc::new(EventBus::new());
    let event_topic = "binance_market_feed";
    let inst = BinanceMarketFeed::new(event_bus.clone(), "indicator_strategy", event_topic);
    tokio::spawn(async move {
        inst.run().await.unwrap();
    });
    event_bus
        .publish(event_topic, Event::MarketFeed(MarketFeedEvent::LoadHistory))
        .unwrap();
    let data_rx = event_bus.subscribe("indicator_strategy".to_owned());
    while let Ok(computed_data) = data_rx.recv() {
        println!("{:?}", computed_data);
    }
}
