use crate::models::*;
use chrono::NaiveDateTime;
use diesel::{result::Error, PgConnection};
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
struct Job {
    timelimit: i64,
    state: String,
    num_cores: i64,
    num_nodes: i64,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
}

impl Job {
    pub fn generate_model(self, id: i32) -> JobstatJob {
        JobstatJob {
            id,
            cluster: Some("test_cluster".to_string()),
            drms_job_id: None,
            drms_task_id: None,
            login: Some(format!("Login {}", id)),
            partition: Some("test".to_string()),
            submit_time: None,
            start_time: Some(self.start_time),
            end_time: Some(self.end_time),
            timelimit: Some(self.timelimit),
            command: Some("./test_command".to_string()),
            state: Some(self.state),
            num_cores: Some(self.num_cores),
            num_nodes: Some(self.num_nodes),
            created_at: chrono::NaiveDate::from_ymd_opt(2010, 10, 10)
                .expect("Failed to create date")
                .and_hms_opt(0, 0, 0)
                .expect("Failed to create date"),
            updated_at: chrono::NaiveDate::from_ymd_opt(2010, 10, 10)
                .expect("Failed to create date")
                .and_hms_opt(0, 0, 0)
                .expect("Failed to create date"),
            nodelist: None,
            initiator_id: None,
        }
    }
}

#[derive(Deserialize, PartialEq, Debug)]
struct Member {
    owner: bool,
    is_access_allowed: bool,
    organization_id: i32,
    user_id: i32,
    jobs: Vec<Job>,
}

impl Member {
    pub fn generate_model(self, id: i32, project_id: i32) -> CoreMember {
        CoreMember {
            id,
            user_id: self.user_id,
            project_id,
            owner: Some(self.owner),
            login: Some(format!("Login {}", id)),
            project_access_state: match self.is_access_allowed {
                true => Some("allowed".to_string()),
                false => Some("suspended".to_string()),
            },
            created_at: chrono::NaiveDate::from_ymd_opt(2010, 10, 10)
                .expect("Failed to create date")
                .and_hms_opt(0, 0, 0),
            updated_at: chrono::NaiveDate::from_ymd_opt(2010, 10, 10)
                .expect("Failed to create date")
                .and_hms_opt(0, 0, 0),
            organization_id: Some(self.organization_id),
            organization_department_id: None,
        }
    }
}

#[derive(Deserialize, PartialEq, Debug)]
struct Project {
    is_active: bool,
    activation_time: NaiveDateTime,
    finish_time: NaiveDateTime,
    estimated_finish_time: NaiveDateTime,
    members: Vec<Member>,
}

impl Project {
    pub fn generate_model(self, id: i32, organization_id: i32) -> CoreProject {
        CoreProject {
            id,
            title: format!("Project {}", id),
            state: match self.is_active {
                true => Some("active".to_string()),
                false => Some("finished".to_string()),
            },
            created_at: chrono::NaiveDate::from_ymd_opt(2010, 10, 10)
                .expect("Failed to create date")
                .and_hms_opt(0, 0, 0),
            updated_at: chrono::NaiveDate::from_ymd_opt(2010, 10, 10)
                .expect("Failed to create date")
                .and_hms_opt(0, 0, 0),
            organization_id: Some(organization_id),
            organization_department_id: None,
            kind_id: None,
            first_activation_at: Some(self.activation_time),
            finished_at: Some(self.finish_time),
            estimated_finish_date: Some(self.estimated_finish_time),
        }
    }
}

impl CoreOrganization {
    pub fn generate_model(id: i32) -> Self {
        Self {
            id,
            name: Some(format!("Organization {}", id)),
            abbreviation: Some(format!("ORG {}", id)),
            kind_id: None,
            country_id: None, //Add mocking
            city_id: None,
            created_at: chrono::NaiveDate::from_ymd_opt(2010, 10, 10)
                .expect("Failed to create date")
                .and_hms_opt(0, 0, 0),
            updated_at: chrono::NaiveDate::from_ymd_opt(2010, 10, 10)
                .expect("Failed to create date")
                .and_hms_opt(0, 0, 0),
            checked: Some(true),
        }
    }
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct GenParameters {
    projects: Vec<Project>,
}

const NUM_ORGANIZATIONS: i32 = 10;
const NUM_USERS: i32 = 10;

impl GenParameters {
    pub fn fill_database(self, connection: &mut PgConnection) -> Result<(), Error> {
        let mut organizations = Vec::new();
        let mut projects = Vec::new();
        let mut members = Vec::new();
        let mut jobs = Vec::new();

        for id in 0..NUM_ORGANIZATIONS {
            organizations.push(CoreOrganization::generate_model(id));
        }

        for (project_id, mut project) in self.projects.into_iter().enumerate() {
            for (member_id, mut member) in project.members.drain(..).enumerate() {
                for (job_id, job) in member.jobs.drain(..).enumerate() {
                    jobs.push(job.generate_model(job_id as i32))
                }
                members.push(member.generate_model(member_id as i32, project_id as i32));
            }
            projects.push(project.generate_model(project_id as i32, 1));
        }

        Ok(())
    }

    pub fn create_from_json(path: &std::path::Path) -> Self {
        let data = std::fs::read_to_string(path).expect("Unable to read file");
        let projects = serde_json::from_str(&data).expect("Unable to deserialize");
        Self { projects }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_deserialize() {
        let data = r#"{
            "timelimit": 1024,
            "state": "COMPLETED",
            "num_cores": 8,
            "num_nodes": 2,
            "start_time": "2002-06-01T00:00:00",
            "end_time": "2002-06-02T00:00:00"
        }"#;
        let job: Job = serde_json::from_str(data).expect("Unable to deserialize");
        let sample_job = Job {
            timelimit: 1024,
            state: "COMPLETED".to_string(),
            num_cores: 8,
            num_nodes: 2,
            start_time: NaiveDateTime::parse_from_str("01.06.2002 00:00:00", "%d.%m.%Y %H:%M:%S")
                .expect("Failed to parse from str"),
            end_time: NaiveDateTime::parse_from_str("02.06.2002 00:00:00", "%d.%m.%Y %H:%M:%S")
                .expect("Failed to parse from str"),
        };
        assert_eq!(job, sample_job);
    }

    #[test]
    fn test_member_deserialize() {
        let data = r#"
        {
            "owner": true,
            "is_access_allowed": true,
            "organization_id": 1,
            "user_id": 1,
            "jobs": [
                {
                "timelimit": 1024,
                "state": "COMPLETED",
                "num_cores": 8,
                "num_nodes": 2,
                "start_time": "2002-06-01T00:00:00",
                "end_time": "2002-06-02T00:00:00"
                }
            ]
        }"#;
        let member: Member = serde_json::from_str(data).expect("Unable to deserialize");
        let sample_job = Job {
            timelimit: 1024,
            state: "COMPLETED".to_string(),
            num_cores: 8,
            num_nodes: 2,
            start_time: NaiveDateTime::parse_from_str("01.06.2002 00:00:00", "%d.%m.%Y %H:%M:%S")
                .expect("Failed to parse from str"),
            end_time: NaiveDateTime::parse_from_str("02.06.2002 00:00:00", "%d.%m.%Y %H:%M:%S")
                .expect("Failed to parse from str"),
        };
        let sample_member = Member {
            owner: true,
            is_access_allowed: true,
            organization_id: 1,
            user_id: 1,
            jobs: vec![sample_job],
        };
        assert_eq!(member, sample_member);
    }


    #[test]
    fn test_project_deserialize() {
        let data = r#"
        {
            "is_active": false,
            "activation_time": "2002-05-01T00:00:00",
            "finish_time": "2002-07-01T00:00:00",
            "estimated_finish_time": "2002-06-01T00:00:00",
            "members": [
                {
                    "owner": true,
                    "is_access_allowed": true,
                    "organization_id": 1,
                    "user_id": 1,
                    "jobs": [
                        {
                        "timelimit": 1024,
                        "state": "COMPLETED",
                        "num_cores": 8,
                        "num_nodes": 2,
                        "start_time": "2002-06-01T00:00:00",
                        "end_time": "2002-06-02T00:00:00"
                        }
                    ]
                }
            ]
        }"#;
        let project: Project = serde_json::from_str(data).expect("Unable to deserialize");
        let sample_job = Job {
            timelimit: 1024,
            state: "COMPLETED".to_string(),
            num_cores: 8,
            num_nodes: 2,
            start_time: NaiveDateTime::parse_from_str("01.06.2002 00:00:00", "%d.%m.%Y %H:%M:%S")
                .expect("Failed to parse from str"),
            end_time: NaiveDateTime::parse_from_str("02.06.2002 00:00:00", "%d.%m.%Y %H:%M:%S")
                .expect("Failed to parse from str"),
        };
        let sample_member = Member {
            owner: true,
            is_access_allowed: true,
            organization_id: 1,
            user_id: 1,
            jobs: vec![sample_job],
        };
        let sample_project = Project {
            is_active: false,
            activation_time: NaiveDateTime::parse_from_str("01.05.2002 00:00:00", "%d.%m.%Y %H:%M:%S")
                .expect("Failed to parse from str"),
            finish_time: NaiveDateTime::parse_from_str("01.07.2002 00:00:00", "%d.%m.%Y %H:%M:%S")
                .expect("Failed to parse from str"),
            estimated_finish_time: NaiveDateTime::parse_from_str("01.06.2002 00:00:00", "%d.%m.%Y %H:%M:%S")
                .expect("Failed to parse from str"),
            members: vec![sample_member]
        };
        assert_eq!(project, sample_project);
    }
}
