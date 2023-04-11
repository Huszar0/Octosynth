use crate::data_analyzer::Analyzer;
use crate::models::*;
use crate::shift_generator::ShiftGenerator;

pub trait Mockable {
    fn mock(self, analyzer: &Analyzer, shift_gen: &ShiftGenerator) -> Self;
}

impl Mockable for JobstatJob {
    fn mock(self, analyzer: &Analyzer, shift_gen: &ShiftGenerator) -> Self {
        let mock_id = self.id + shift_gen.id_shifts["jobstat_jobs"] as i32;
        Self {
            id: mock_id,
            cluster: self.cluster, //should be mocked?
            drms_job_id: self
                .drms_job_id
                .and_then(|id| Some(id + shift_gen.id_shifts["drms_jobs"])), //what is it?
            drms_task_id: self
                .drms_task_id
                .and_then(|id| Some(id + shift_gen.id_shifts["drms_tasks"])),
            login: self.initiator_id.and_then(|id| {
                Some(format!(
                    "Login {}",
                    id + shift_gen.id_shifts["core_members"] as i32
                ))
            }),
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
            timelimit: Some(analyzer.generate_value("timelimit") as i64),
            command: Some(format!("command {}", mock_id)), //TODO: should command be mocked?
            state: self.state,
            num_cores: Some(analyzer.generate_value("num_cores") as i64),
            num_nodes: Some(analyzer.generate_value("num_nodes") as i64),
            created_at: self
                .created_at
                .checked_sub_months(shift_gen.time_shift)
                .unwrap(),
            updated_at: self
                .updated_at
                .checked_sub_months(shift_gen.time_shift)
                .unwrap(),
            nodelist: self.nodelist, //What to do??
            initiator_id: self //Is JobstatJob????
                .initiator_id
                .and_then(|id| Some(id + shift_gen.id_shifts["core_members"] as i32)),
        }
    }
}

impl Mockable for CoreProject {
    fn mock(self, _analyzer: &Analyzer, shift_gen: &ShiftGenerator) -> Self {
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
    fn mock(self, _analyzer: &Analyzer, shift_gen: &ShiftGenerator) -> Self {
        let mock_id = self.id + shift_gen.id_shifts["core_members"] as i32;
        Self {
            id: mock_id,
            user_id: self.user_id,
            project_id: self.project_id + shift_gen.id_shifts["core_project"] as i32,
            owner: self.owner,
            login: Some(format!("Login {}", mock_id)),
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
    fn mock(self, _analyzer: &Analyzer, shift_gen: &ShiftGenerator) -> Self {
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
    fn mock(self, _analyzer: &Analyzer, shift_gen: &ShiftGenerator) -> Self {
        let mock_id = self.id + shift_gen.id_shifts["organizations_departments"] as i32;
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
    fn mock(self, _analyzer: &Analyzer, shift_gen: &ShiftGenerator) -> Self {
        let mock_id = self.id + shift_gen.id_shifts["organizations_kinds"] as i32;
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
    fn mock(self, _analyzer: &Analyzer, shift_gen: &ShiftGenerator) -> Self {
        let mock_id = self.id + shift_gen.id_shifts["jobstat_float_data"] as i32;
        Self {
            id: mock_id,
            name: self.name,
            job_id: self.job_id.and_then(|id| Some(id + shift_gen.id_shifts["jobstat_jobs"])),
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
    fn mock(self, _analyzer: &Analyzer, shift_gen: &ShiftGenerator) -> Self {
        let mock_id = self.id + shift_gen.id_shifts["jobstat_string_data"] as i32;
        Self {
            id: mock_id,
            name: self.name,
            job_id: self.job_id.and_then(|id| Some(id + shift_gen.id_shifts["jobstat_jobs"])),
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
