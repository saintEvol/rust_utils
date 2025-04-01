
#[cfg(feature = "kameo")]
use kameo::Actor;
#[cfg(feature = "kameo")]
use crate::global_holder::GlobalHolder;

#[cfg(feature = "kameo")]
pub type  GlobalActorHolder<A> = GlobalHolder<kameo::actor::ActorRef<A>>;
