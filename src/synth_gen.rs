use crate::models::*;
use chrono::{NaiveDateTime};
use diesel::{result::Error, PgConnection};
use serde::Deserialize; 

#[derive(Deserialize)]
struct Job {
    timelimit: usize,
    state: String,
    num_cores: i64,
    num_node: i64,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
}

#[derive(Deserialize)]
struct Member {
    owner: bool,
    is_access_allowed: bool,
    organization: i32,
    jobs: Vec<Job>,
}

#[derive(Deserialize)]
struct Project {
    members: Vec<Member>,
    is_active: bool,
    activation_time: NaiveDateTime,
    finish_time: NaiveDateTime,
    estimated_finish_time: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct GenParameters {
    projects: Vec<Project>
}

impl GenParameters {
    pub fn fill_database(connection: &mut PgConnection) -> Result<(), Error> {
        
        Ok(())
    }

    pub fn create_from_json(path: &std::path::Path) -> Self {
        let data = std::fs::read_to_string(path).expect("Unable to read file");
        let projects = serde_json::from_str(&data).expect("Undable to deserialize");
        Self { projects }
    }
}







