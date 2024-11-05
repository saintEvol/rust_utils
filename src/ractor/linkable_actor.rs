#![cfg(feature = "ractor")]
use ractor::{Actor, ActorCell, ActorRef, SpawnErr};
use ractor::concurrency::JoinHandle;

///
trait LinkableActor: Actor {
    async fn spawn_linked(
        args: <Self as Actor>::Arguments,
        supervisor: ActorCell,
    ) -> Result<(ActorRef<Self::Msg>, JoinHandle<()>), SpawnErr>;

}

#[cfg(test)]
mod test {
    use ractor::{Actor, ActorCell, ActorProcessingErr, ActorRef, SpawnErr};
    use ractor::concurrency::JoinHandle;
    use crate::ractor::linkable_actor::LinkableActor;

    struct TestActor;

    impl Actor for TestActor {
        type Msg = ();
        type State = ();
        type Arguments = ();

        async fn pre_start(&self, myself: ActorRef<Self::Msg>, args: Self::Arguments) -> Result<Self::State, ActorProcessingErr> {
            todo!()
        }
    }

    impl LinkableActor for TestActor {
        async fn spawn_link(args: <Self as Actor>::Arguments, supervisor: ActorCell) -> Result<(ActorRef<Self::Msg>, JoinHandle<()>), SpawnErr> {
            todo!()
        }
    }

}

