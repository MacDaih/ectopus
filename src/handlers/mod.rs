use serde_json;
use warp::{
    Reply,
    Rejection,
    http::StatusCode
};
use mongodb::error::Error;

use crate::db;
use crate::db::service::{ReaderService};
use crate::db::model::{Report, Run};
mod utils;

async fn run_service() -> Result<ReaderService, Error> {
    let service = db::db()
        .await
        .map_err(|err| println!("{:?}", err));
    match service {
        Ok(service) => Ok(service),
        Err(e) => panic!("{:?}",e),
    }
}

pub async fn read(j: serde_json::Value) -> Result<impl Reply, Rejection> {
    let res = j.get("runs").unwrap();

    let runs = res.as_array().unwrap();
    let mut run_results = Vec::new();
    for r in runs {
        let result = create_new_suite(r);
        run_results.push(result);
    }

    let project = j.get("name").unwrap();
    let start = j.get("start_at").unwrap();
    let dur = j.get("duration").unwrap();
    let total_t = j.get("total_tests").unwrap();
    let fail = j.get("failed").unwrap();
    let pass = j.get("passed").unwrap();

    let report = Report{
        name: serde_json::from_value(project.to_owned()).unwrap(),
        start_at: serde_json::from_value(start.to_owned()).unwrap(),
        duration: serde_json::from_value(dur.to_owned()).unwrap(),
        total_tests: serde_json::from_value(total_t.to_owned()).unwrap(),
        failed: serde_json::from_value(fail.to_owned()).unwrap(),
        passed: serde_json::from_value(pass.to_owned()).unwrap(),
        runs: run_results,
    };
    
    let init = run_service().await.unwrap();
    let _flag = init.insert_report(&report).await.unwrap();
    Ok(StatusCode::OK)
}

fn create_new_suite(s: &serde_json::Value) -> Run {
    let tsts = s.get("tests").unwrap();
    let spec = s.get("spec").unwrap();
    let reporter_stats = s.get("reporter_stats").unwrap();
    Run{
        tests: serde_json::from_value(tsts.to_owned()).unwrap(),
        spec: serde_json::from_value(spec.to_owned()).unwrap(),
        stats: serde_json::from_value(reporter_stats.to_owned()).unwrap(),
    }
}

pub async fn retrieve_reports() -> Result<impl Reply, Rejection>  {
    let init = run_service().await.unwrap();
    let results = init.get_all(None).await.unwrap();
    Ok(warp::reply::json(&results))
}

pub async fn filter_reports(s: serde_json::Value) -> Result<impl Reply, Rejection> {
    let doc = utils::set_filter(s);
    let init = run_service().await.unwrap();
    let results = init.get_all(Some(doc)).await.unwrap();
    Ok(warp::reply::json(&results))
}
pub async fn retrieve_metrics() -> Result<impl Reply, Rejection> {
    let init = run_service().await.unwrap();
    let results = init.get_metrics().await.unwrap();
    Ok(warp::reply::json(&results))
}