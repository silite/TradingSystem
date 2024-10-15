use protocol::{market::Market, portfolio::position::MetaPosition};

pub trait PositionHandler {
    /// 获取仓位id。
    fn position_id(&self) -> &Market;

    /// 设置可用仓位
    fn set_open_position(&mut self, position: MetaPosition) -> anyhow::Result<()>;

    /// diff可用仓位
    fn diff_open_position(&mut self, position: MetaPosition) -> anyhow::Result<()>;

    /// 获取所有可用仓位，正为多仓、负为空仓。
    fn get_open_positions(&self) -> anyhow::Result<&Vec<MetaPosition>>;

    /// 设置在途仓位
    fn set_freezed_position(&mut self, position: MetaPosition) -> anyhow::Result<()>;

    /// diff在途仓位
    fn diff_freezed_position(&mut self, position: MetaPosition) -> anyhow::Result<()>;

    /// 获取在途、冻结、挂单中的仓位，区分正负。
    fn get_freezed_positions(&self) -> anyhow::Result<&Vec<MetaPosition>>;

    /// 移除仓位
    fn remove_position(&mut self) -> anyhow::Result<()>;

    /// 设置退出交易的仓位。
    fn set_exited_position(&mut self, position: MetaPosition) -> anyhow::Result<()>;

    /// diff退出交易的仓位。
    fn diff_exited_position(&mut self, position: MetaPosition) -> anyhow::Result<()>;

    /// 获取所有退出交易的仓位。
    fn get_exited_positions(&self) -> anyhow::Result<&Vec<MetaPosition>>;
}
