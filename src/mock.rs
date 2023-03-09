use std::collections::HashMap;

use chrono::Months;

use crate::models::CoreMember;
use crate::models::CoreProject;
use crate::models::JobstatJob;

impl JobstatJob {
    pub fn mock(self, averages: &HashMap<String, i64>, id_shifts: &HashMap<String, i64>) -> Self {
        let mock_id = self.id + id_shifts["jobstat_jobs"] as i32;
        Self {
            id: mock_id,
            cluster: Some(format!("cluster №{}", mock_id)),
            drms_job_id: self
                .drms_job_id
                .and_then(|id| Some(id + id_shifts["drms_job"])),
            drms_task_id: self
                .drms_task_id
                .and_then(|id| Some(id + id_shifts["drms_task"])),
            login: self.login,
            partition: Some(format!("partion №{}", mock_id)),
            submit_time: self
                .submit_time
                .and_then(|time| time.checked_sub_months(Months::new(100 as u32))),
            start_time: self
                .start_time
                .and_then(|time| time.checked_sub_months(Months::new(100 as u32))),
            end_time: self
                .end_time
                .and_then(|time| time.checked_sub_months(Months::new(100 as u32))),
            timelimit: Some(averages["timelimit"]),
            command: Some(format!("command №{}", mock_id)),
            state: self.state,
            num_cores: Some(averages["num_cores"]),
            num_nodes: Some(averages["num_nodes"]),
            created_at: self
                .created_at
                .checked_sub_months(Months::new(100 as u32))
                .unwrap(),
            updated_at: self
                .updated_at
                .checked_sub_months(Months::new(100 as u32))
                .unwrap(),
            nodelist: Some(format!("nodelist №{}", mock_id)),
            initiator_id: self
                .initiator_id
                .and_then(|id| Some(id + id_shifts["initiator"] as i32)),
        }
    }
}

impl CoreProject {
    pub fn mock(self, _averages: &HashMap<String, i64>, id_shifts: &HashMap<String, i64>) -> Self {
        let mock_id = self.id + id_shifts["core_project"] as i32;
        Self {
            id: mock_id,
            title: format!("project №{}", mock_id),
            state: self.state,
            created_at: self
                .created_at
                .and_then(|time| time.checked_sub_months(Months::new(200 as u32))),
            updated_at: self
                .updated_at
                .and_then(|time| time.checked_sub_months(Months::new(200 as u32))),
            organization_id: self
                .organization_id
                .and_then(|x| Some(x + id_shifts["organization"] as i32)),
            organization_department_id: self
                .organization_department_id
                .and_then(|x| Some(x + id_shifts["organizationdepartment"] as i32)),
            kind_id: self.kind_id.and_then(|x| Some(x + 200)),
            first_activation_at: self
                .first_activation_at
                .and_then(|time| time.checked_sub_months(Months::new(100 as u32))),
            finished_at: self
                .finished_at
                .and_then(|time| time.checked_sub_months(Months::new(100 as u32))),
            estimated_finish_date: self
                .estimated_finish_date
                .and_then(|time| time.checked_sub_months(Months::new(100 as u32))),
        }
    }
}

impl CoreMember {
    pub fn mock(self, _averages: &HashMap<String, i64>, id_shifts: &HashMap<String, i64>) -> Self {
        let mock_id = self.id + id_shifts["core_member"] as i32;
        Self {
            id: mock_id,
            user_id: self.user_id + mock_id,
            project_id: self.project_id + mock_id,
            owner: self.owner,
            login: self.login,
            project_access_state: Some(format!("project access state №{}", mock_id)),
            created_at: self
                .created_at
                .and_then(|time| time.checked_sub_months(Months::new(300 as u32))),
            updated_at: self
                .updated_at
                .and_then(|time| time.checked_sub_months(Months::new(300 as u32))),
            organization_id: self
                .organization_id
                .and_then(|id| Some(id + id_shifts["organization"] as i32)),
            organization_department_id: self
                .organization_department_id
                .and_then(|id| Some(id + id_shifts["organization_department"] as i32)),
        }
    }
}
