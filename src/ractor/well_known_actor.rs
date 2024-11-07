

#[cfg(feature = "ractor")]
/// 一个被全局知晓的Actor进程，可以获取到其进程id
pub trait WellKnownActor: ractor::Actor {
    /// 获取自己的全局注册名
    fn global_name() -> ractor::ActorName;

    /// 获取自己的进程id
    fn me() -> Option<ractor::ActorRef<<Self as ractor::Actor>::Msg>> {
        ractor::registry::where_is(<Self as WellKnownActor>::global_name()).map(|cell|cell.into())
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