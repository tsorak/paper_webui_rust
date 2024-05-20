use axum::{routing::get, Router};

use crate::routes::{about, root};

mod components;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // A normal axum router...
    let app = Router::new()
        .route("/", get(root::root))
        .route("/about", get(about::about))
        // Use a precompiled and minified build of axum-live-view's JavaScript.
        // This is the easiest way to get started. Integration with bundlers
        // is of course also possible.
        .route("/assets/live-view.js", axum_live_view::precompiled_js());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await?;

    Ok(())
}
