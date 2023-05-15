use migration::sea_orm::DatabaseConnection;
use tonic::Status;

pub trait GetDatabaseFromRequest {
    fn get_db(&self) -> Result<&DatabaseConnection, Status>;
}

/// Get database from request Generics are used
impl<T> GetDatabaseFromRequest for tonic::Request<T> {
    fn get_db(&self) -> Result<&DatabaseConnection, Status> {
        self.extensions()
            .get::<DatabaseConnection>()
            .ok_or(Status::internal("no database found"))
    }
}
