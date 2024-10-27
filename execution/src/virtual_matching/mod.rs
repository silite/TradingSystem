use crate::ExecutionExt;

#[derive(Clone)]
pub struct Matching;

impl ExecutionExt for Matching {
    fn new_order(&self) -> anyhow::Result<()> {
        todo!()
    }

    fn cancel_order(&self) -> anyhow::Result<()> {
        todo!()
    }

    fn query_all_order(&self) -> anyhow::Result<()> {
        todo!()
    }

    fn query_all_trade(&self) -> anyhow::Result<()> {
        todo!()
    }
}
