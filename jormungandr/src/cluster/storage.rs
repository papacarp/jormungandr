//use actix_raft::RaftStorage;
//
///// Your application's storage interface actor.
//struct AppStorage {/* ... snip ... */}
//
//// Ensure you impl this over your application's data, data response & error types.
//impl RaftStorage<Data, DataResponse, Error> for AppStorage {
//    type Actor = Self;
//    type Context = Context<Self>;
//}
//
//impl Actor for AppStorage {
//    type Context = Context<Self>;
//}
//
//impl Handler<storage::GetInitialState<Error>> for AppStorage {
//    type Result = ResponseActFuture<Self, storage::InitialState, Error>;
//
//    fn handle(
//        &mut self,
//        _msg: storage::GetInitialState<Error>,
//        _ctx: &mut Self::Context,
//    ) -> Self::Result {
//        // ... snip ...
//    }
//}
