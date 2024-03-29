use crate::models::*;
use crate::shift_generator::ShiftGenerator;
use diesel::prelude::*;
use diesel::PgConnection;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

pub trait Mockable {
    fn mock(self, shift_gen: &ShiftGenerator) -> Self;
}

impl Mockable for JobstatJob {
    fn mock(self, shift_gen: &ShiftGenerator) -> Self {
        let mock_id = self.id + shift_gen.id_shifts["jobstat_jobs"] as i32;
        let mut hasher = DefaultHasher::new();
        let cluster = self.cluster.and_then(|cluster_name| {
            hasher.write(cluster_name.as_bytes());
            Some(format!("Cluster {}", hasher.finish()))
        });
        hasher = DefaultHasher::new();
        let login = self.login.and_then(|login| {
            hasher.write(login.as_bytes());
            Some(format!("Login {}", hasher.finish()))
        });
        Self {
            id: mock_id,
            cluster,
            drms_job_id: self.drms_job_id,
            drms_task_id: self.drms_task_id,
            login,
            partition: self.partition,
            submit_time: self
                .submit_time
                .and_then(|time| time.checked_sub_months(shift_gen.time_shift)),
            start_time: self
                .start_time
                .and_then(|time| time.checked_sub_months(shift_gen.time_shift)),
            end_time: self
                .end_time
                .and_then(|time| time.checked_sub_months(shift_gen.time_shift)),
            timelimit: self.timelimit,
            command: Some(format!("command {}", mock_id)),
            state: self.state,
            num_cores: self.num_cores,
            num_nodes: self.num_nodes,
            created_at: self
                .created_at
                .checked_sub_months(shift_gen.time_shift)
                .unwrap(),
            updated_at: self
                .updated_at
                .checked_sub_months(shift_gen.time_shift)
                .unwrap(),
            nodelist: self.nodelist,
            initiator_id: self
                .initiator_id
                .and_then(|id| Some(id + shift_gen.id_shifts["jobstat_jobs"] as i32)),
        }
    }
}

impl Mockable for CoreProject {
    fn mock(self, shift_gen: &ShiftGenerator) -> Self {
        let mock_id = self.id + shift_gen.id_shifts["core_projects"] as i32;
        Self {
            id: mock_id,
            title: format!("project {}", mock_id),
            state: self.state,
            created_at: self
                .created_at
                .and_then(|time| time.checked_sub_months(shift_gen.time_shift)),
            updated_at: self
                .updated_at
                .and_then(|time| time.checked_sub_months(shift_gen.time_shift)),
            organization_id: self
                .organization_id
                .and_then(|x| Some(x + shift_gen.id_shifts["organizations"] as i32)),
            organization_department_id: self
                .organization_department_id
                .and_then(|x| Some(x + shift_gen.id_shifts["organization_departments"] as i32)),
            kind_id: self.kind_id,
            first_activation_at: self
                .first_activation_at
                .and_then(|time| time.checked_sub_months(shift_gen.time_shift)),
            finished_at: self
                .finished_at
                .and_then(|time| time.checked_sub_months(shift_gen.time_shift)),
            estimated_finish_date: self
                .estimated_finish_date
                .and_then(|time| time.checked_sub_months(shift_gen.time_shift)),
        }
    }
}

impl Mockable for CoreMember {
    fn mock(self, shift_gen: &ShiftGenerator) -> Self {
        let mock_id = self.id + shift_gen.id_shifts["core_members"] as i32;
        let mut hasher = DefaultHasher::new();
        let login = self.login.and_then(|login| {
            hasher.write(login.as_bytes());
            Some(format!("Login {}", hasher.finish()))
        });
        Self {
            id: mock_id,
            user_id: self.user_id,
            project_id: self.project_id + shift_gen.id_shifts["core_projects"] as i32,
            owner: self.owner,
            login,
            project_access_state: self.project_access_state,
            created_at: self
                .created_at
                .and_then(|time| time.checked_sub_months(shift_gen.time_shift)),
            updated_at: self
                .updated_at
                .and_then(|time| time.checked_sub_months(shift_gen.time_shift)),
            organization_id: self
                .organization_id
                .and_then(|id| Some(id + shift_gen.id_shifts["organizations"] as i32)),
            organization_department_id: self
                .organization_department_id
                .and_then(|id| Some(id + shift_gen.id_shifts["organization_departments"] as i32)),
        }
    }
}

impl Mockable for CoreOrganization {
    fn mock(self, shift_gen: &ShiftGenerator) -> Self {
        let mock_id = self.id + shift_gen.id_shifts["organizations"] as i32;
        Self {
            id: mock_id,
            name: Some(format!("Organization {}", mock_id)),
            abbreviation: Some(format!("ORG {}", mock_id)),
            kind_id: self
                .kind_id
                .and_then(|id| Some(id + shift_gen.id_shifts["organization_kinds"] as i32)),
            country_id: self.country_id,
            city_id: self.city_id,
            created_at: self
                .created_at
                .and_then(|time| time.checked_sub_months(shift_gen.time_shift)),
            updated_at: self
                .updated_at
                .and_then(|time| time.checked_sub_months(shift_gen.time_shift)),
            checked: self.checked,
        }
    }
}

impl Mockable for CoreOrganizationDepartment {
    fn mock(self, shift_gen: &ShiftGenerator) -> Self {
        let mock_id = self.id + shift_gen.id_shifts["organization_departments"] as i32;
        Self {
            id: mock_id,
            organization_id: self
                .organization_id
                .and_then(|id| Some(id + shift_gen.id_shifts["organizations"] as i32)),
            name: Some(format!("Organization department {}", mock_id)),
            checked: self.checked,
        }
    }
}

impl Mockable for CoreOrganizationKind {
    fn mock(self, shift_gen: &ShiftGenerator) -> Self {
        let mock_id = self.id + shift_gen.id_shifts["organization_kinds"] as i32;
        Self {
            id: mock_id,
            name_ru: Some(format!("Тип организации {}", mock_id)),
            departments_required: self.departments_required,
            created_at: self
                .created_at
                .and_then(|time| time.checked_sub_months(shift_gen.time_shift)),
            updated_at: self
                .updated_at
                .and_then(|time| time.checked_sub_months(shift_gen.time_shift)),
            name_en: Some(format!("Organization kind {}", mock_id)),
        }
    }
}

impl Mockable for JobstatFloatData {
    fn mock(self, shift_gen: &ShiftGenerator) -> Self {
        let mock_id = self.id + shift_gen.id_shifts["jobstat_float_data"] as i32;
        Self {
            id: mock_id,
            name: self.name,
            job_id: self
                .job_id
                .and_then(|id| Some(id + shift_gen.id_shifts["jobstat_jobs"])),
            value: self.value,
            created_at: self
                .created_at
                .checked_sub_months(shift_gen.time_shift)
                .unwrap_or(self.created_at),
            updated_at: self
                .created_at
                .checked_sub_months(shift_gen.time_shift)
                .unwrap_or(self.created_at),
        }
    }
}

impl Mockable for JobstatStringData {
    fn mock(self, shift_gen: &ShiftGenerator) -> Self {
        let mock_id = self.id + shift_gen.id_shifts["jobstat_string_data"] as i32;
        Self {
            id: mock_id,
            name: self.name,
            job_id: self
                .job_id
                .and_then(|id| Some(id + shift_gen.id_shifts["jobstat_jobs"])),
            value: self.value,
            created_at: self
                .created_at
                .checked_sub_months(shift_gen.time_shift)
                .unwrap_or(self.created_at),
            updated_at: self
                .created_at
                .checked_sub_months(shift_gen.time_shift)
                .unwrap_or(self.created_at),
        }
    }
}

impl Mockable for CoreCity {
    fn mock(self, shift_gen: &ShiftGenerator) -> Self {
        let mock_id = self.id + shift_gen.id_shifts["core_cities"] as i32;
        Self {
            id: mock_id,
            country_id: self
                .country_id
                .and_then(|id| Some(id + shift_gen.id_shifts["core_countries"] as i32)),
            title_ru: Some(format!("Город {}", mock_id)),
            title_en: Some(format!("City {}", mock_id)),
            checked: self.checked,
        }
    }
}

impl Mockable for CoreCountry {
    fn mock(self, shift_gen: &ShiftGenerator) -> Self {
        let mock_id = self.id + shift_gen.id_shifts["core_countries"] as i32;
        Self {
            id: mock_id,
            title_ru: Some(format!("Страна {}", mock_id)),
            title_en: Some(format!("Country {}", mock_id)),
            checked: self.checked,
        }
    }
}

macro_rules! depersonalize {
    ($conn_in:expr, $table_in:expr, $conn_out:expr, $table_out:expr, $model:ty, $shift_gen:expr) => {
        let records = $table_in
            .load::<$model>($conn_in)
            .expect("Failed to load jobstat_jobs");
        let deperson_records: Vec<_> = records
            .into_iter()
            .map(|job| job.mock(&$shift_gen))
            .collect();

        let size_of_batch = 2000;
        let mut left_border = 0;

        for _ in 0..(deperson_records.len() / size_of_batch + 1) {
            let mut right_border = left_border + size_of_batch;
            if right_border > deperson_records.len() {
                right_border = deperson_records.len();
            }

            diesel::insert_into($table_out)
                .values(&deperson_records[left_border..right_border])
                .execute($conn_out)
                .expect("Error saving to new database");

            left_border += size_of_batch;
        }
    };
}

pub fn fill_database(
    connection_in: &mut PgConnection,
    connection_out: &mut PgConnection,
    shift_gen: ShiftGenerator,
) {
    use crate::schema::core_cities::dsl::*;
    use crate::schema::core_countries::dsl::*;
    use crate::schema::core_members::dsl::*;
    use crate::schema::core_organization_departments::dsl::*;
    use crate::schema::core_organization_kinds::dsl::*;
    use crate::schema::core_organizations::dsl::*;
    use crate::schema::core_projects::dsl::*;
    use crate::schema::jobstat_float_data::dsl::*;
    use crate::schema::jobstat_jobs::dsl::*;
    use crate::schema::jobstat_string_data::dsl::*;

    depersonalize!(
        connection_in,
        jobstat_jobs,
        connection_out,
        crate::schema::jobstat_jobs::table,
        JobstatJob,
        shift_gen
    );

    depersonalize!(
        connection_in,
        core_projects,
        connection_out,
        crate::schema::core_projects::table,
        CoreProject,
        shift_gen
    );

    depersonalize!(
        connection_in,
        core_members,
        connection_out,
        crate::schema::core_members::table,
        CoreMember,
        shift_gen
    );

    depersonalize!(
        connection_in,
        core_organizations,
        connection_out,
        crate::schema::core_organizations::table,
        CoreOrganization,
        shift_gen
    );

    depersonalize!(
        connection_in,
        core_organization_kinds,
        connection_out,
        crate::schema::core_organization_kinds::table,
        CoreOrganizationKind,
        shift_gen
    );

    depersonalize!(
        connection_in,
        core_organization_departments,
        connection_out,
        crate::schema::core_organization_departments::table,
        CoreOrganizationDepartment,
        shift_gen
    );

    depersonalize!(
        connection_in,
        core_cities,
        connection_out,
        crate::schema::core_cities::table,
        CoreCity,
        shift_gen
    );

    depersonalize!(
        connection_in,
        core_countries,
        connection_out,
        crate::schema::core_countries::table,
        CoreCountry,
        shift_gen
    );

    depersonalize!(
        connection_in,
        jobstat_float_data,
        connection_out,
        crate::schema::jobstat_float_data::table,
        JobstatFloatData,
        shift_gen
    );

    depersonalize!(
        connection_in,
        jobstat_string_data,
        connection_out,
        crate::schema::jobstat_string_data::table,
        JobstatStringData,
        shift_gen
    );
}
