use serde_json;
use bson::{doc, Document};

const NAME: &str = "name";
const FAILED: &str = "failed";
const RANGE_DUR: &str = "range_dur";
const RANGE_TIME: &str = "range_time";

pub fn set_filter(s: serde_json::Value) -> Document {
    let obj = s.as_object().unwrap();
    let mut d = doc!{};
    for (i,v) in obj.iter() {
        match i {
            i if i == &NAME =>d.insert("name",v.as_str().unwrap()),
            i if i == &FAILED => match v.as_bool().unwrap() {
                    true => d.insert("failed", doc!{ "$gt" : 0}),
                    false => d.insert("passed", doc!{ "$gt" : 0}),
                },
            i if i == &RANGE_DUR => {
                let min = v.get("min").unwrap();
                let max = v.get("max").unwrap();
                d.insert("duration", doc!{
                    "$lt": min.as_i64().unwrap(),
                    "$gt": max.as_i64().unwrap(),
                })
            }
            i if i == &RANGE_TIME => {
                let min = v.get("min").unwrap();
                let max = v.get("max").unwrap();
                d.insert("start_at", doc!{
                    "$lt": min.to_string(),
                    "$gt": max.to_string(),
                })
            } 
            &_ => d.insert("nope".to_string(),"nope".to_string()),
        };
    }
    return d;
}