use chrono::Months;
use rand::Rng;
use std::collections::HashMap;

const CLASS_NAMES: [&str; 8] = [
    "jobstat_jobs",
    "core_projects",
    "core_members",
    "drms_job",
    "drms_task",
    "initiators",
    "core_organizations",
    "core_organization_departments",
];
const MAX_SHIFT: i64 = 1000;
const MIN_TIME_SHIFT: i64 = 500;
const MAX_TIME_SHIFT: i64 = 1000;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen() {
        let gen = ShiftGenerator::new();
        for (k, v) in gen.id_shifts {
            println!("{} {}", k, v)
        }
    }
}
