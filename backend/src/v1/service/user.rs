#[derive(Debug, Default)]
pub struct UserController {}

impl proto::user_service_server::UserService for UserController {}
