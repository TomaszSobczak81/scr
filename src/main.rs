use std::path::Path;
use tokio::net::UnixListener;
use tokio_stream::wrappers::UnixListenerStream;

use crate::k8s::runtime::v1::runtime_service_server::RuntimeServiceServer;

pub mod k8s {
    #[path = ""]
    pub mod runtime {
        #[path = "runtime.v1.rs"]
        pub mod v1;
    }
}

pub mod scr;

#[cfg(unix)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prepare the path for the unix socket
    let path = "/tmp/scr.sock";

    std::fs::create_dir_all(Path::new(path).parent().unwrap())?;

    // Create an instance of the implementation of the gRPC service
    let runtime = crate::scr::runtime::v1::RuntimeService::default();

    // Create unix socket
    let uds = UnixListener::bind(path)?;
    let uds_stream = UnixListenerStream::new(uds);

    // Create a gRPC server
    tonic::transport::Server::builder()
        .add_service(RuntimeServiceServer::new(runtime))
        .serve_with_incoming(uds_stream)
        .await?;

    Ok(())
}

#[cfg(not(unix))]
fn main() {
    panic!("This software only works on unix systems!");
}
