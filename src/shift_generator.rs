use chrono::Months;
use rand::Rng;
use std::collections::HashMap;

const CLASS_NAMES: [&str; 10] = [
    "jobstat_jobs",
    "core_projects",
    "core_members",
    "organizations",
    "organization_departments",
    "organization_kinds",
    "jobstat_string_data",
    "jobstat_float_data",
    "core_cities",
    "core_countries",
];
const MAX_SHIFT: i64 = 1000;
const MIN_TIME_SHIFT: i64 = 5;
const MAX_TIME_SHIFT: i64 = 500;
pub struct ShiftGenerator {
    pub id_shifts: HashMap<&'static str, i64>,
    pub time_shift: Months,
}

impl ShiftGenerator {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut id_shifts = HashMap::new();
        for class_name in CLASS_NAMES {
            id_shifts.insert(class_name, rng.gen_range(0..MAX_SHIFT));
        }
        let time_shift = Months::new(rng.gen_range(MIN_TIME_SHIFT..MAX_TIME_SHIFT) as u32);
        Self {
            id_shifts,
            time_shift,
        }
    }
}
