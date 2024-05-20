use axum::response::IntoResponse;
use axum_live_view::{html, LiveViewUpgrade};

use crate::components::Counter;

pub async fn root(live: LiveViewUpgrade) -> impl IntoResponse {
    let counter = Counter::default();

    live.response(|embed_live_view| {
        html! {
            <!DOCTYPE html>
            <html>
                <head>
                </head>
                <body>
                    <nav>
                        <a href="/">"root"</a>
                        <a href="/about">"about"</a>
                    </nav>
                    <h1>{"Home"}</h1>
                    {embed_live_view.embed(counter)}
                    <script src="/assets/live-view.js"></script>
                </body>
            </html>
        }
    })
}
