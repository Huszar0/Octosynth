#![recursion_limit = "512"]
#[macro_use]
extern crate diesel;
extern crate bigdecimal;
extern crate chrono;
extern crate dotenvy;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use mock::fill_database;
use std::env;
use synth_gen::GenParameters;

mod mock;
mod models;
mod schema;
mod shift_generator;
mod synth_gen;

pub fn establish_connection(database_url: &str) -> PgConnection {
    dotenv().ok();
    let database_url = env::var(database_url).expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn clear_database(connection: &mut PgConnection, tables_names: &[&str]) {
    for table_name in tables_names {
        diesel::sql_query(format!("TRUNCATE TABLE {} CASCADE", table_name))
            .execute(connection)
            .expect(&("Error truncating the table".to_owned() + table_name));
    }

    
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut connection_in = establish_connection("DATABASE_IN_URL");
    let mut connection_out = establish_connection("DATABASE_OUT_URL");

    if args.len() < 2 {
        panic!("Set type of work trough command line arguments");
    }
    if args[1].starts_with("d") {
        let shift_gen = shift_generator::ShiftGenerator::new();

        let tables_names: [&str; 10] = [
            "core_projects",
            "core_members",
            "jobstat_jobs",
            "core_organizations",
            "core_organization_departments",
            "core_organization_kinds",
            "jobstat_float_data",
            "jobstat_string_data",
            "core_cities",
            "core_countries",
        ];

        clear_database(&mut connection_out, &tables_names);

        fill_database(&mut connection_in, &mut connection_out, shift_gen);
    } else if args[1].starts_with("s") {
        let data = std::fs::read_to_string("./src/gen_params.json")
            .expect("Unable to read file with parametrs");
        let parameters: GenParameters =
            serde_json::from_str(&data).expect("Unable to deserialize parametrs");

        let tables_names: [&str; 5] = [
            "core_projects",
            "core_members",
            "jobstat_jobs",
            "core_organizations",
            "users",
        ];

        clear_database(&mut connection_out, &tables_names);

        parameters
            .fill_database(&mut connection_out)
            .expect("Failed to generate synthetic data");
    } else {
        panic!("Wrong type of work");
    }
}
