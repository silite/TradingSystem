#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
//! 信息收集模块
//! 不在主线程执行
use std::{collections::BTreeMap, sync::LazyLock};

use message::MonitorMessage;
use ractor::{concurrency::JoinHandle, Actor, ActorProcessingErr, ActorRef};

pub mod message;

pub struct MonitorActor;
pub struct MonitorState {}

pub static MONITOR: LazyLock<ActorRef<MonitorMessage>> = LazyLock::new(|| {
    std::thread::Builder::new()
        .name("%monitor".to_string())
        .spawn(move || {
            let monitor = tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async move {
                    let (monitor, _monitor_actor_handle) = MonitorActor::new().await;
                    monitor
                });
            monitor
        })
        .unwrap()
        .join()
        .unwrap()
});

impl MonitorActor {
    pub async fn new() -> (ActorRef<MonitorMessage>, JoinHandle<()>) {
        Actor::spawn(None, MonitorActor, ())
            .await
            .expect("Failed to start monitor actor")
    }
}

#[ractor::async_trait]
impl Actor for MonitorActor {
    type State = MonitorState;
    type Arguments = ();
    type Msg = MonitorMessage;

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        _arguments: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(MonitorState {})
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {}
    }
}
