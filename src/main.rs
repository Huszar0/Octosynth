#![recursion_limit = "512"]
#[macro_use]
extern crate diesel;
extern crate bigdecimal;
extern crate chrono;
extern crate dotenvy;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

mod mock;
mod models;
mod schema;
mod shift_generator;
mod synth_gen;

use mock::Mockable;
use models::JobstatJob;
use schema::jobstat_jobs::dsl::*;

pub fn establish_connection(database_url: &str) -> PgConnection {
    dotenv().ok();
    let database_url = env::var(database_url).expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    let mut connection_in = establish_connection("DATABASE_IN_URL");
    let mut connection_out = establish_connection("DATABASE_OUT_URL");
    let shift_gen = shift_generator::ShiftGenerator::new();

    let mut results = jobstat_jobs
        .load::<JobstatJob>(&mut connection_in)
        .expect("Error loading members");
    let mut jobs_num_cores = Vec::with_capacity(results.len());
    let mut jobs_num_nodes = Vec::with_capacity(results.len());
    let mut jobs_timelimit = Vec::with_capacity(results.len());
    for job in results.iter() {
        if job.num_cores.is_some() {
            jobs_num_cores.push(job.num_cores.unwrap() as f64);
        }
        if job.num_nodes.is_some() {
            jobs_num_nodes.push(job.num_nodes.unwrap() as f64);
        }
        if job.timelimit.is_some() {
            jobs_timelimit.push(job.timelimit.unwrap() as f64);
        }
    }

    diesel::sql_query("TRUNCATE jobstat_jobs")
        .execute(&mut connection_out)
        .expect("Error truncating the table");

    for job in results {
        println!("{:?}", job.mock(&shift_gen));
    }
    /*     diesel::insert_into(jobstat_jobs::table)
    .values(&results)
    .execute(&mut connection_out)
    .expect("Error saving to new database"); */
}
