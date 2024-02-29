use crate::k8s::runtime::v1::runtime_service_server;

#[derive(Debug, Default)]
pub struct RuntimeService;

#[tonic::async_trait]
impl runtime_service_server::RuntimeService for RuntimeService {
    // async fn some_rpc_method(
    //     &self,
    //     request: tonic::Request<k8s::runtime::v1::SomeRequest>,
    // ) -> Result<tonic::Response<k8s::runtime::v1::SomeResponse>, tonic::Status> {
    //     // Implement the logic for the RPC method
    //     unimplemented!()
    // }
}
