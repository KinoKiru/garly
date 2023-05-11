use crate::proto;
use tonic::{Request, Status};

#[derive(Debug, Default)]
pub struct UserController {}

#[tonic::async_trait]
impl proto::user_service_server::UserService for UserController {
    /// List all users
    async fn list_users(&self, req: Request<ListUserRequest>) -> Result<ListUserResponse, Status> {
        todo!()
    }

    /// Get specific user by id
    async fn get_user(&self, req: Request<GetUserRequest>) -> Result<Response<User>, Status> {
        todo!()
    }

    /// Create a user, this is basically the register function
    async fn create_user(&self, req: Request<CreateUserRequest>) -> Result<Response<User>, Status> {
        todo!()
    }

    /// Update a user, Works as a patch so whole object is not necessary but a id is.
    async fn update_user(&self, req: Request<UpdateUserRequest>) -> Result<Response<User>, Status> {
        todo!()
    }

    /// Remove a user based on id
    async fn delete_user(&self, req: Request<DeleteUserRequest>) -> Result<Response<()>, Status> {
        todo!()
    }

    /// Login the user with either email or username and a password
    async fn login_user(
        &self,
        req: Request<LoginUserRequest>,
    ) -> Result<LoginUserResponse, Status> {
        todo!()
    }
}
