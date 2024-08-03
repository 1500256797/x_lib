#[warn(async_fn_in_trait)]
use anyhow::Result;
use reqwest::{Client, RequestBuilder, Response};
pub trait IntoRequestBuilder {
    fn into_request(self, client: Client) -> RequestBuilder;
}
pub trait SendRequestAndLog {
    // async fn send_request_and_log(self) -> Result<Response>;
    fn send_request_and_log(self) -> impl std::future::Future<Output = Result<Response>> + Send;
}
