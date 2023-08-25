mod user {
    tonic::include_proto!("user");
}

use tonic::{Request, Response, Status};
use tonic_reflection::server::Builder;
use user::{
    user_service_server::{UserService, UserServiceServer},
    UserRequest, UserResponse,
};

pub struct MyUserService {}

#[tonic::async_trait]
impl UserService for MyUserService {
    async fn say_user(
        &self,
        request: Request<UserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let res = UserResponse {
            message: format!("User, {}!", request.into_inner().name),
        };
        Ok(Response::new(res))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    let user_service = MyUserService {};

    tonic::transport::Server::builder()
        .add_service(UserServiceServer::new(user_service))
        .add_service(
            Builder::configure()
                .register_encoded_file_descriptor_set(tonic::include_file_descriptor_set!(
                    "user_descriptor"
                ))
                .build()
                .unwrap(),
        )
        .serve(addr)
        .await?;

    Ok(())
}
