#[cfg(feature = "ractor")]
use ractor::{Actor, ActorCell, ActorRef, SpawnErr};
#[cfg(feature = "ractor")]
use ractor::concurrency::JoinHandle;

#[cfg(feature = "ractor")]
/// 一个被全局知晓的Actor进程，可以获取到其进程id
pub trait WellKnownActor: ractor::Actor {
    /// 获取自己的全局注册名
    fn global_name() -> ractor::ActorName;

    /// 获取自己的进程id
    fn me() -> Option<ActorRef<<Self as ractor::Actor>::Msg>> {
        ractor::registry::where_is(<Self as WellKnownActor>::global_name()).map(|cell|cell.into())
    }

    async fn spawn_linked(self, args: <Self as Actor>::Arguments, supervisor: ActorCell) -> Result<(ActorRef<Self::Msg>, JoinHandle<()>), SpawnErr> {
        Actor::spawn_linked(Some(Self::global_name()), self, args, supervisor)
    }
    // fn cast(msg: <Self as ractor::Actor>::Msg) {
    //     use ractor::{cast, ActorRef};
    //
    //     let me: ActorRef<<Self as ractor::Actor>::Msg>
    //         = <Self as WellKnownActor>::me().map(|cell|cell.into()).unwrap();
    //     cast!(me, msg);
    // }
    //
    // fn call(msg: <Self as ractor::Actor>::Msg) {
    //     use ractor::{call, ActorRef};
    //
    //     let me: ActorRef<<Self as ractor::Actor>::Msg>
    //         = <Self as WellKnownActor>::me().map(|cell|cell.into()).unwrap();
    //
    //     call!(me, )
    // }

}