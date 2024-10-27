pub mod virtual_matching;

pub trait ExecutionExt {
    fn new_order(&self) -> anyhow::Result<()>;

    fn cancel_order(&self) -> anyhow::Result<()>;

    fn query_all_order(&self) -> anyhow::Result<()>;

    fn query_all_trade(&self) -> anyhow::Result<()>;
}
