use crate::data_analyzer::Analyzer;
use crate::models::CoreMember;
use crate::models::CoreProject;
use crate::models::JobstatJob;
use crate::shift_generator::ShiftGenerator;

impl JobstatJob {
    pub fn mock(self, analyzer: &Analyzer, shift_gen: &ShiftGenerator) -> Self {
        let mock_id = self.id + shift_gen.id_shifts["jobstat_jobs"] as i32;
        Self {
            id: mock_id,
            cluster: self.cluster,
            drms_job_id: self
                .drms_job_id
                .and_then(|id| Some(id + shift_gen.id_shifts["drms_job"])),
            drms_task_id: self
                .drms_task_id
                .and_then(|id| Some(id + shift_gen.id_shifts["drms_task"])),
            login: self.initiator_id.and_then(|id| {
                Some(format!(
                    "Login {}",
                    id + shift_gen.id_shifts["initiator"] as i32
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
            initiator_id: self
                .initiator_id
                .and_then(|id| Some(id + shift_gen.id_shifts["initiator"] as i32)),
        }
    }
}

impl CoreProject {
    pub fn mock(self, _analyzer: &Analyzer, shift_gen: &ShiftGenerator) -> Self {
        let mock_id = self.id + shift_gen.id_shifts["core_project"] as i32;
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
                .and_then(|x| Some(x + shift_gen.id_shifts["organization"] as i32)),
            organization_department_id: self
                .organization_department_id
                .and_then(|x| Some(x + shift_gen.id_shifts["organization_department"] as i32)),
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

impl CoreMember {
    pub fn mock(self, _analyzer: &Analyzer, shift_gen: &ShiftGenerator) -> Self {
        let mock_id = self.id + shift_gen.id_shifts["core_member"] as i32;
        Self {
            id: mock_id,
            user_id: self.user_id + mock_id,
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
                .and_then(|id| Some(id + shift_gen.id_shifts["organization"] as i32)),
            organization_department_id: self
                .organization_department_id
                .and_then(|id| Some(id + shift_gen.id_shifts["organization_department"] as i32)),
        }
    }
}
