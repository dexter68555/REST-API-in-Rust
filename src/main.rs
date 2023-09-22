use warp;

mod model;
mod db;
mod handler;
mod route;

#[tokio::main]
async fn main() {
    let db = db::init_db();
    let customer_routes = route::customer_routes(db);

    warp::serve(customer_routes)
        .run(([127, 0, 0, 1], 3000))
        .await;
}