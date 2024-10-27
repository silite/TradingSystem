use std::{
    cell::{Cell, RefCell},
    ops::Deref,
    rc::Rc,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use crossbeam::channel::{Receiver, Sender};
use derive_builder::Builder;
use futures::FutureExt;
use protocol::{
    event::{bus::CommandBus, DataKind, MarketDataEvent, MarketFeedCommand, TradeEvent},
    indictor::Indicators,
    portfolio::market_data::binance::Kline,
};
use tokio::sync::{mpsc, oneshot};
use utils::runtime::TOKIO_RUNTIME;

use crate::{
    indictor::{ComputedIndicators, Indicator},
    MarketFeed,
};

#[derive(Clone)]
pub struct BinanceMarketFeed {
    computed_indicators: ComputedIndicators,
    command_bus: Arc<CommandBus>,
    command_topic: &'static str,
    data_tx: Sender<(Kline, Indicators)>,
}

impl MarketFeed for BinanceMarketFeed {
    type MarketData = Kline;

    fn new(
        command_bus: Arc<CommandBus>,
        command_topic: &'static str,
    ) -> crossbeam::channel::Receiver<(Self::MarketData, Indicators)> {
        let (data_tx, data_rx) = crossbeam::channel::unbounded();
        let mut inst = Self {
            computed_indicators: ComputedIndicators::new(),
            command_bus,
            command_topic,
            data_tx,
        };
        TOKIO_RUNTIME.spawn(async move {
            inst.run()
                .await
                .map_err(|err| ftlog::error!("[market feed] run error. {:?}", err))
                // 必须run起来
                .unwrap();
        });
        data_rx
    }

    async fn run(&mut self) -> anyhow::Result<()> {
        ftlog::info!("[binance market feed] run.");
        let mut command_rx = self.command_bus.subscribe(self.command_topic.to_owned());
        while let Some(command) = command_rx.recv().await {
            match command {
                protocol::event::Command::MarketFeed(market_feed_command) => {
                    match market_feed_command {
                        MarketFeedCommand::LoadHistory => match self
                            .load_history_market_data()
                            .await
                        {
                            Ok(history_cnt) => {
                                ftlog::info!("Load market data history done. cnt({})", history_cnt);
                            }
                            Err(err) => {
                                ftlog::error!(
                                    "[market feed] load_history_market_data error. {:?}",
                                    err
                                )
                            }
                        },
                    }
                }
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

        let select_join = TOKIO_RUNTIME.spawn(async {
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
        let dc = self.computed_indicators.dc.push(&market_data).get();
        let rsi = self.computed_indicators.rsi.push(&market_data).get();
        let ema = self.computed_indicators.ema.push(&market_data).get();
        let stoch_rsi = self.computed_indicators.stock_rsi.push(rsi).get(rsi);
        let adx = self.computed_indicators.adx.push(&market_data).get();
        let macd = self.computed_indicators.macd.push(&market_data).get();
        let tr = self.computed_indicators.tr.push(&market_data).get();
        let tr_rma = self.computed_indicators.tr_rma.push(tr).get();

        let atr_low = market_data.low - self.computed_indicators.pre_tr_rma * 1.5;
        let atr_high = self.computed_indicators.pre_tr_rma * 1.5 + market_data.high;
        self.computed_indicators.pre_tr_rma = tr_rma;

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

        if let Err(e) = self.data_tx.send((market_data, indicators)) {
            ftlog::error!("send computed indicator error {}.", e);
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 6)]
async fn test_market_feed() {
    utils::logs::logs_guard();
    let command_bus = Arc::new(CommandBus::new());
    let command_topic = "binance_market_feed";
    let data_rx = BinanceMarketFeed::new(command_bus.clone(), command_topic);

    sleep(Duration::from_millis(500));

    command_bus
        .publish(
            command_topic,
            protocol::event::Command::MarketFeed(MarketFeedCommand::LoadHistory),
        )
        .unwrap();
    while let Ok(computed_data) = data_rx.recv() {
        println!("{:?}", computed_data);
    }
}
