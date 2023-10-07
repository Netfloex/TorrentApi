use serde::Serialize;
use std::sync::Mutex;
use surf::{
    middleware::{Middleware, Next},
    Body, Client, Error, Request, Response, Result, Url,
};

#[derive(Serialize)]
struct Credentials {
    username: String,
    password: String,
}

impl Credentials {
    fn new<S: Into<String>, P: Into<String>>(username: S, password: P) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }
}

pub struct AuthMiddleware {
    url: Url,
    session_id: Mutex<Option<String>>,
    username: String,
    password: String,
}

impl AuthMiddleware {
    pub async fn login(&self, client: &Client) -> Result<String> {
        println!("Logging in");
        let form = Credentials::new(&self.username, &self.password);
        let body = Body::from_form(&form).unwrap();

        let resp = client
            .post("/api/v2/auth/login")
            .body(body)
            .header("origin", self.url.to_string())
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(Error::from_str(
                resp.status(),
                resp.status().canonical_reason(),
            ));
        }

        if let Some(cookie) = resp.header("set-cookie").map(|c| c.get(0)).flatten() {
            let session_id = cookie.as_str().split(";").next().unwrap().to_string();
            *self.session_id.lock().unwrap() = Some(session_id.clone());

            return Ok(session_id);
        }

        Err(Error::from_str(401, "No cookie"))
    }

    pub fn new(username: String, password: String, url: Url) -> Self {
        Self {
            url,
            session_id: Mutex::new(None),
            username,
            password,
        }
    }

    fn session_id(&self) -> Option<String> {
        self.session_id.lock().unwrap().clone()
    }
}

#[surf::utils::async_trait]
impl Middleware for AuthMiddleware {
    async fn handle(
        &self,
        mut req: Request,
        client: Client,
        next: Next<'_>,
    ) -> surf::Result<Response> {
        let session_id = match self.session_id() {
            Some(session_id) => session_id,
            None => self.login(&client).await?,
        };

        req.insert_header("Cookie", session_id);

        let res = next.run(req, client).await?;
        Ok(res)
    }
}
