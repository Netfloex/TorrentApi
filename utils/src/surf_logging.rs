use surf::middleware::{Middleware, Next};
use surf::{Client, Request, Response};

pub struct SurfLogging;

#[surf::utils::async_trait]
impl Middleware for SurfLogging {
    async fn handle(&self, req: Request, client: Client, next: Next<'_>) -> surf::Result<Response> {
        println!("{} \"{}\"", req.method(), req.url());
        let res = next.run(req, client).await?;
        Ok(res)
    }
}
