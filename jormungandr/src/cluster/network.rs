use crate::cluster::data::Data;
use actix::{Actor, Context, Handler, ResponseActFuture};
use actix_raft::messages::{
    AppendEntriesRequest, AppendEntriesResponse, InstallSnapshotRequest, InstallSnapshotResponse,
    VoteRequest, VoteResponse,
};
use actix_raft::RaftNetwork;

struct AppNetwork {}

impl Actor for AppNetwork {
    type Context = Context<Self>;
}

// Ensure you impl this over your application's data type. Here, it is `Data`.
impl RaftNetwork<Data> for AppNetwork {}

impl Handler<VoteRequest> for AppNetwork {
    type Result = ResponseActFuture<Self, VoteResponse, ()>;

    fn handle(&mut self, _msg: VoteRequest, _ctx: &mut Self::Context) -> Self::Result {
        unimplemented!()
    }
}

impl Handler<AppendEntriesRequest<Data>> for AppNetwork {
    type Result = ResponseActFuture<Self, AppendEntriesResponse, ()>;

    fn handle(
        &mut self,
        _msg: AppendEntriesRequest<Data>,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        unimplemented!()
    }
}

impl Handler<InstallSnapshotRequest> for AppNetwork {
    type Result = ResponseActFuture<Self, InstallSnapshotResponse, ()>;

    fn handle(&mut self, _msg: InstallSnapshotRequest, _ctx: &mut Self::Context) -> Self::Result {
        unimplemented!()
    }
}
