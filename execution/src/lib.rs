use error::ExecutionError;
use protocol::order::{OrderRequest, OrderResponse};

pub mod error;
pub mod virtual_matching;

pub trait ExecutionExt {
    fn new_order(
        &self,
        order: OrderRequest,
        order_cb_tx: crossbeam::channel::Sender<anyhow::Result<OrderResponse>>,
    ) -> impl std::future::Future<Output = anyhow::Result<(), ExecutionError>> + Send;

    fn cancel_order(
        &self,
    ) -> impl std::future::Future<Output = anyhow::Result<(), ExecutionError>> + Send;

    fn query_all_order(
        &self,
    ) -> impl std::future::Future<Output = anyhow::Result<(), ExecutionError>> + Send;

    fn query_all_trade(
        &self,
    ) -> impl std::future::Future<Output = anyhow::Result<(), ExecutionError>> + Send;
}
