use derive_builder::Builder;

use crate::{
    market::symbol::Instrument,
    portfolio::{amount::Amount, volume::Volume},
    trade::Side,
};

#[derive(Debug, Clone)]
pub struct OrderRequest {
    pub main_order: Order,          // 主单
    pub take_profit: Option<Order>, // 止盈单
    pub stop_loss: Option<Order>,   // 止损单
}

#[derive(Builder, Debug, Clone)]
pub struct Order {
    pub side: Side,
    pub order_type: OrderType,
    pub volume: f64,
    pub price: Option<f64>,
    // pub time_in_force: TimeInForce,
}

#[derive(Debug, Clone)]
pub enum OrderType {
    Market,

    Limit,

    Stop,

    // 止损限价单 - 当价格达到触发价时，以指定限价卖出/买入
    StopLimit {
        stop_price: f64,
        limit_price: f64,
    },

    // 追踪止损单 - 价格每上涨，止损价相应上调
    TrailingStop {
        callback_rate: f64, // 回调比例
    },
}

#[derive(Debug, Clone)]
pub enum OrderResponse {
    OrderSuccess(Order),
    OrderError(Order),
}
