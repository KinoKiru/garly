use tonic::{Request, Status};

/// Log the incoming request
pub fn logger(req: Request<()>) -> Result<Request<()>, Status> {
    struct User {
        username: String,
    }
    let logged_in = req.extensions().get::<User>();

    info!(
        "[{}] ({})",
        req.remote_addr().map_or(String::new(), |ip| ip.to_string()),
        logged_in.map_or("#anon#".to_string(), |user| user.username.clone()),
    );

    Ok(req)
}
