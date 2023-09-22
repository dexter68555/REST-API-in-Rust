use std::convert::Infallible;
use warp::{self, Filter};

use crate::db::Db;
use crate::handler;
use crate::model::Customer;

// All customer routes
pub fn customer_routes(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_customer(db.clone())
        .or(update_customer(db.clone()))
        .or(delete_customer(db.clone()))
        .or(create_customer(db.clone()))
        .or(customers_list(db))
}

// GET customers
fn customers_list(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("getCustomer")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handler::list_customers)
}

// POST Create customers
fn create_customer(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("createCustomer")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handler::create_customer)
}

// GET /customers/{id}
fn get_customer(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("getCustomerWithId" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handler::get_customer)
}

// PUT update customer
fn update_customer(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("updateCustomer" / String)
        .and(warp::put())
        .and(json_body())
        .and(with_db(db))
        .and_then(handler::update_customer)
}

// DELETE customer
fn delete_customer(
    db: Db
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("deleteCustomer" / String)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(handler::delete_customer)
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body() -> impl Filter<Extract = (Customer,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
