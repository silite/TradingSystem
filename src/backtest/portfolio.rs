use portfolio::{MetaPortfolio, MetaPortfolioBuilder};
use protocol::market::Market;
use uuid::Uuid;

pub fn init_portfolio(engine_id: Uuid, market: Market) -> MetaPortfolio {
    MetaPortfolioBuilder::default()
        .engine_id(engine_id)
        .market(market)
        .open_balance(Default::default())
        .freezed_balance(Default::default())
        .exited_balance(Default::default())
        .open_position(Default::default())
        .freezed_position(Default::default())
        .exited_position(Default::default())
        .update_ms(Default::default())
        .build()
        .expect("init portfolio error.")
}
