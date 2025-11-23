pub mod proto {
    // The path is relative to the current file
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/users.rs"));
}

use tokio_stream::Stream;
use std::pin::Pin;
#[derive(Debug,Clone,Copy,Default)]
pub struct UserService{

}
#[tonic::async_trait]
impl proto::user_guide_server::UserGuide for UserService {
    // ---- REQUIRED ----
    type GetUsersStream =
        Pin<Box<dyn Stream<Item = Result<proto::User, tonic::Status>> + Send + 'static>>;

    async fn get_users(
        &self,
        request: tonic::Request<()>,
    ) -> Result<tonic::Response<Self::GetUsersStream>, tonic::Status> {
        println!("received: {:?}", request);

        // Build a simple stream of users
        let stream = tokio_stream::iter(vec![
            Ok(proto::User {
                id: 1,
                name: "John".into(),
                age: 20,
                is_active: 1,
            }),
            Ok(proto::User {
                id: 2,
                name: "Alice".into(),
                age: 22,
                is_active: 1,
            }),
        ]);

        Ok(tonic::Response::new(Box::pin(stream)))
    }

    // --------------------------------------------------------

    async fn add_user(
        &self,
        request: tonic::Request<proto::AddUserMssg>,
    ) -> Result<tonic::Response<proto::User>, tonic::Status> {
        let user = proto::User {
            id: 2,
            name: "john".into(),
            age: 4,
            is_active: 0,
        };

        Ok(tonic::Response::new(user))
    }

    async fn update_user(
        &self,
        request: tonic::Request<proto::UpdateUserMssg>,
    ) -> Result<tonic::Response<proto::User>, tonic::Status> {
        let user = proto::User {
            id: 2,
            name: "john".into(),
            age: 4,
            is_active: 1,
        };

        Ok(tonic::Response::new(user))
    }
}

#[tokio::main]
async fn main() {
   
    let addr="[::1]:50051".parse().unwrap();
    let user=UserService::default();
    use tonic::transport::Server;

    Server::builder()
        .add_service(proto::user_guide_server::UserGuideServer::new(user))
        .serve(addr)
        .await
        .unwrap();
    
}
// to suggest the function should be inlined
#[inline]
fn some()
{

}

// #to always inline the function no matter the cost

#[inline(always)]
fn some2()
{
    
}