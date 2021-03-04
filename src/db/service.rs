use mongodb::{
    Cursor,
    error::Error,
    options::{FindOptions},
    results::InsertOneResult,
    Collection,
};

use bson::{doc, Document};
use futures::stream::StreamExt;

use super::model::{
    Report,
    Linear,
    Run,
    Figures,
    Metric,
    Speed
};

pub struct ReaderService {
    pub collection: Collection,
}

impl ReaderService {
    pub fn new(c: Collection) -> ReaderService {
        ReaderService {
            collection: c
        }
    }
    pub async fn insert_report(&self, r: &Report) -> Result<InsertOneResult, Error> {
        self.collection.insert_one(report_to_doc(r), None).await
    }
    pub async fn get_all(&self, filters: Option<Document>) -> Result<Vec<Report>, Error> {
        let find_options = FindOptions::builder().sort(doc! { "_id": -1 }).build();
        let mut cursor: Cursor;
        match filters {
            Some(f) => cursor = self.collection.find(f, find_options).await.unwrap(),
            None => cursor = self.collection.find(None, find_options).await.unwrap(),
        }
        let mut results: Vec<Report> = Vec::new();
        while let Some(doc) = cursor.next().await {    
            results.push(doc_to_report(&doc?).unwrap());
        };
        Ok(results)
    }

    pub async fn get_metrics(&self) -> Result<Metric, Error> {
        let group = doc! {
                "$group": {
                    "_id": { "$dayOfMonth": "$start_at" },
                    "total": { "$sum": "$total_tests" },
                    "passed": { "$sum": "$passed" },
                    "failed": { "$sum": "$failed" },
                }
        };
        let mut cursor = self.collection.aggregate(vec![group, sort()], None).await.unwrap();
        let mut results: Vec<Linear> = Vec::new();
        while let Some(doc) = cursor.next().await {
            match doc {
                Ok(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
                    Ok(res) => results.push(res),
                    Err(err) => println!("{}", err)
                },
                Err(err) => println!("{}",err),
            }
        }
        let fig = self.get_figures().await.unwrap();
        let speed = self.get_speed().await.unwrap();
        Ok(Metric{
            linear: results,
            figures: fig,
            speed: speed,
        })
    }

    async fn get_figures(&self) -> Result<Vec<Figures>, Error> {
        let figure = doc! {
            "$group": {
                "_id": "$name",
                "total": { "$sum": "$total_tests" },
                "highest": {
                    "$max": "$passed"
                },
                "lowest": {
                    "$min": "$passed"
                }
            }
        };
        let mut cursor = self.collection.aggregate(vec![figure, sort()], None).await.unwrap();
        let mut results: Vec<Figures> = Vec::new();

        while let Some(doc) = cursor.next().await {
            match doc {
                Ok(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
                    Ok(res) => results.push(res),
                    Err(err) => println!("{}", err)
                },
                Err(err) => println!("{}",err),
            }
        }
        Ok(results)
    }
    async fn get_speed(&self) -> Result<Vec<Speed>, Error> {
        let speed = doc! {
            "$group": {
                "_id": "$name",
                "fastest": {
                    "$min": "$duration"
                },
                "average": {
                    "$avg": "$duration"
                },
                "slowest": {
                    "$max": "$duration"
                }
            }
        };
        let mut cursor = self.collection.aggregate(vec![speed, sort()], None).await.unwrap();
        let mut speeds: Vec<Speed> = Vec::new();
        while let Some(doc) = cursor.next().await {
            match doc {
                Ok(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
                    Ok(res) => speeds.push(res),
                    Err(err) => println!("{}", err)
                },
                Err(err) => println!("{}",err),
            }
        }
        Ok(speeds)
    }
}

fn report_to_doc(r: &Report) -> Document {
    let runs = bson::to_bson(&r.runs).unwrap();
    doc! {
        "name": r.name.to_owned(),
        "start_at": r.start_at.to_owned(),
        "duration": r.duration.to_owned(),
        "total_tests": r.total_tests.to_owned(),
        "failed": r.failed.to_owned(),
        "passed": r.passed.to_owned(),
        "runs": runs,
    }
}

fn doc_to_report(doc: &Document) -> Result<Report, Error> {
    let name = doc.get_str("name").unwrap();
    let start_at = doc.get_datetime("start_at").unwrap();
    let duration = doc.get_i32("duration").unwrap();
    let total = doc.get_i32("total_tests").unwrap();
    let failed = doc.get_i32("failed").unwrap();
    let passed = doc.get_i32("passed").unwrap();
    let b_runs = doc.get_array("runs").unwrap().to_owned();
    let mut v_runs: Vec<Run> = Vec::new();
    for res in b_runs.iter(){
        v_runs.push(bson::from_bson(res.to_owned()).unwrap());
    }

    let r = Report {
        name: name.to_owned(),
        start_at: start_at.to_owned(),
        duration: duration.to_owned(),
        total_tests: total.to_owned(),
        failed: failed.to_owned(),
        passed: passed.to_owned(),
        runs: v_runs,
    };
    Ok(r)
}

fn sort() -> Document {
    doc! {
        "$sort" : { 
            "_id" : 1 
        }
    }
}