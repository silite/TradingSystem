use protocol::order::{OrderRequest, OrderResponse};

use crate::{error::ExecutionError, ExecutionExt};

#[derive(Clone)]
pub struct VirtualMatching {
    order_resp_tx: crossbeam::channel::Sender<OrderResponse>,
    order_resp_rx: crossbeam::channel::Receiver<OrderResponse>,
}

impl ExecutionExt for VirtualMatching {
    fn new() -> Self {
        let (order_resp_tx, order_resp_rx) = crossbeam::channel::unbounded();
        Self {
            order_resp_tx,
            order_resp_rx,
        }
    }

    fn get_order_resp_rx(&self) -> crossbeam::channel::Receiver<OrderResponse> {
        self.order_resp_rx.clone()
    }

    async fn new_order(&self, order: OrderRequest) -> anyhow::Result<(), ExecutionError> {
        let msg = OrderResponse::OrderSuccess(order.main_order);
        Ok(self.order_resp_tx.send(msg)?)
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
