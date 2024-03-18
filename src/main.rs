use proto::combo_server::{Combo, ComboServer};
use tonic::transport::Server;

mod proto {
    tonic::include_proto!("combo");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("combo_descriptor");
}

#[derive(Debug, Default)]
struct ComboService {}

#[tonic::async_trait]
impl Combo for ComboService {
    async fn sol(
        &self,
        request: tonic::Request<proto::ComboRequest>,
    ) -> Result<tonic::Response<proto::ComboResponse>, tonic::Status> {
        println!("Got a request: {:?}", request);

        let _input = request.get_ref(); //TODO: Actual Calculation

        let response = proto::ComboResponse { damage: 420 };

        Ok(tonic::Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let combo = ComboService::default();

    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(reflection)
        .add_service(ComboServer::new(combo))
        .serve(addr)
        .await?;

    Ok(())
}
