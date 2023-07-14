
use async_trait::async_trait;
use crate::grpc::nacos_proto::Payload;
use crate::grpc::{PayloadHandler, PayloadUtils, RequestMeta};
use crate::raft::NacosRaft;
use crate::raft::store::innerstore::InnerStore;
use crate::raft::store::{NodeId, TypeConfig};

pub struct RaftVoteRequestHandler {
    raft : NacosRaft,
}

impl RaftVoteRequestHandler{
    pub fn new(raft : NacosRaft) -> Self {
        Self{
            raft
        }
    }
}

#[async_trait]
impl PayloadHandler for RaftVoteRequestHandler {
    async fn handle(&self, request_payload: Payload, _request_meta: RequestMeta) -> anyhow::Result<Payload> {
        let body_vec = request_payload.body.unwrap_or_default().value;
        let request: openraft::raft::VoteRequest<NodeId> = serde_json::from_slice(&body_vec)?;
        let res = self.raft.vote(request).await;
        match res {
            Ok(res) => {
                let value = serde_json::to_string(&res)?;
                let payload = PayloadUtils::build_payload("RaftVoteResponse",value );
                Ok(payload)
            }
            Err(e) => {
                let value = serde_json::to_string(&e)?;
                let payload = PayloadUtils::build_error_payload( 500u16,value);
                Ok(payload)
            }
        }
    }
}