use warp::Filter;

#[tokio::main]
async fn main() {
    let routes = warp::fs::dir("pkg")
        .or(warp::fs::dir("static"))
        .or(warp::fs::file("index.html"));

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}