use prest::*;

fn main() {
    Router::new()
        .route("/", get(homepage))
        .serve(Default::default())
}

async fn homepage() -> Markup {
    html!(h1{"Hello world"})
}
