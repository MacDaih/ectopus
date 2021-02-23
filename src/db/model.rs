use serde::{Deserialize,Serialize};
use chrono::{ DateTime, Utc };

#[derive(Debug, Deserialize, Serialize)]
pub struct Report {
    pub name: String,
    pub start_at: DateTime<Utc>,
    pub duration: i32,
    pub total_tests: i32,
    pub failed: i32,
    pub passed: i32,
    pub runs: Vec<Run>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Run {
    pub tests: Vec<Test>,
    pub spec: Spec,
    pub stats: Stats,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Spec {
    pub name:   String,
    pub relative: String,
    pub absolute: String,
    pub spec_type:   String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Test {
    pub title:  Vec<String>,
    pub state:  String,
    pub body:   String,
    pub attempts:   Vec<Attempt>
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Attempt {
    pub state:  String,
    pub duration:   i32,
    pub started_at:  DateTime<Utc>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Stats {
    pub suites: i32,
    pub tests:  i32,
    pub passes: i32,
    pub pending:    i32,
    pub failures:   i32,
    pub start:   DateTime<Utc>,
    pub end:     DateTime<Utc>,
    pub duration:   i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Metric {
    pub linear: Vec<Linear>,
    pub figures: Vec<Figures>,
    pub speed: Vec<Speed>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Linear {
    pub _id: i32,
    pub total: i32,
    pub passed: i32,
    pub failed: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Figures {
    pub _id: String,
    pub total: i32,
    pub highest: i32,
    pub lowest: i32 
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Speed {
    pub _id: String,
    pub fastest: i32,
    pub average: f64,
    pub slowest: i32,
}