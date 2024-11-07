
#[cfg(feature = "ractor")]
/// 一个被全局知晓的Actor进程，可以获取到其进程id
pub trait WellKnownActor: ractor::Actor {
    /// 获取自己的全局注册名
    fn global_name(self) -> ractor::ActorName;

    /// 获取自己的进程id
    fn me(self) -> Option<ractor::ActorRef<<Self as ractor::Actor>::Msg>> {
        ractor::registry::where_is(self.global_name()).map(|cell|cell.into())
    }

}