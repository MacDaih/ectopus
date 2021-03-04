extern crate tokio;

use warp::{
    Filter
};

mod db;
mod handlers;

#[tokio::main]
async fn main() {
    let add_report = warp::path("new")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handlers::read);

    let get_metrics = warp::path("metrics")
        .and(warp::get())
        .and_then(handlers::retrieve_metrics);

    let get_reports = warp::path("reports")
        .and(warp::get())
        .and_then(handlers::retrieve_reports);

    let filter = warp::path("filter")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handlers::filter_reports);

    //FIXME becareful of cors security matters
    // set here to allow all for dev purpose
    let routes = add_report
        .or(get_reports).with(warp::cors().allow_any_origin())
        .or(get_metrics).with(warp::cors().allow_any_origin())
        .or(filter).with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}