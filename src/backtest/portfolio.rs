use portfolio::{MetaPortfolio, MetaPortfolioBuilder};
use uuid::Uuid;

pub fn init_portfolio(engine_id: Uuid) -> MetaPortfolio {
    MetaPortfolioBuilder::default()
        .build()
        .expect("init portfolio error.")
}
