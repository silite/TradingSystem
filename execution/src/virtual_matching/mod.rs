use protocol::order::{OrderRequest, OrderResponse};

use crate::{error::ExecutionError, ExecutionExt};

#[derive(Clone)]
pub struct Matching;

impl ExecutionExt for Matching {
    async fn new_order(
        &self,
        order: OrderRequest,
        order_cb_tx: crossbeam::channel::Sender<anyhow::Result<OrderResponse>>,
    ) -> anyhow::Result<(), ExecutionError> {
        ftlog::info!("[Execution] Order Success. {:?}", order);
        let msg = OrderResponse::OrderSuccess(order.main_order);
        Ok(order_cb_tx.send(Ok(msg))?)
    }

    async fn cancel_order(&self) -> anyhow::Result<(), ExecutionError> {
        todo!()
    }

    async fn query_all_order(&self) -> anyhow::Result<(), ExecutionError> {
        todo!()
    }

    async fn query_all_trade(&self) -> anyhow::Result<(), ExecutionError> {
        todo!()
    }
}
