use vercel_runtime::{bundled_api, run, Body, Error, Request, Response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

// The proc macro `bundled_api` injects a router for all `api/**/*.rs` handler files .
// If you are using cargo workspaces (like `examples/route-merge` in this repository),
// then an additional `path` argument must be passed to the macro. E.g.
// #[bundled_api( path = "examples/route-merge" )]
#[bundled_api]
pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    
}