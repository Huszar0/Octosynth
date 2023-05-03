use std::collections::HashMap;

use crate::models::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
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
    pub fn generate_model(self, id: i32, member_id: i32, project_id: i32) -> JobstatJob {
        JobstatJob {
            id,
            cluster: Some("test_cluster".to_string()),
            drms_job_id: None,
            drms_task_id: None,
            login: Some(format!("Login {}_{}", member_id, project_id)),
            partition: Some("test".to_string()),
            submit_time: None,
            start_time: Some(self.start_time),
            end_time: Some(self.end_time),
            timelimit: Some(self.timelimit),
            command: Some("./test_command".to_string()),
            state: Some(self.state),
            num_cores: Some(self.num_cores),
            num_nodes: Some(self.num_nodes),
            created_at: chrono::NaiveDate::from_ymd_opt(2022, 10, 10)
                .expect("Failed to create date")
                .and_hms_opt(0, 0, 0)
                .expect("Failed to create date"),
            updated_at: chrono::NaiveDate::from_ymd_opt(2022, 10, 10)
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
            login: Some(format!("Login {}_{}", self.user_id, project_id)),
            project_access_state: match self.is_access_allowed {
                true => Some("allowed".to_string()),
                false => Some("suspended".to_string()),
            },
            created_at: chrono::NaiveDate::from_ymd_opt(2022, 10, 10)
                .expect("Failed to create date")
                .and_hms_opt(0, 0, 0),
            updated_at: chrono::NaiveDate::from_ymd_opt(2022, 10, 10)
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
            created_at: chrono::NaiveDate::from_ymd_opt(2022, 10, 10)
                .expect("Failed to create date")
                .and_hms_opt(0, 0, 0),
            updated_at: chrono::NaiveDate::from_ymd_opt(2022, 10, 10)
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
            country_id: None,
            city_id: None,
            created_at: chrono::NaiveDate::from_ymd_opt(2022, 10, 10)
                .expect("Failed to create date")
                .and_hms_opt(0, 0, 0),
            updated_at: chrono::NaiveDate::from_ymd_opt(2022, 10, 10)
                .expect("Failed to create date")
                .and_hms_opt(0, 0, 0),
            checked: Some(true),
        }
    }
}

impl User {
    pub fn generate_model(id: i32) -> Self {
        Self {
            id,
            email: format!("email{}@octoshell.ru", id),
            crypted_password: None,
            salt: None,
            created_at: chrono::NaiveDate::from_ymd_opt(2022, 10, 10)
                .expect("Failed to create date")
                .and_hms_opt(0, 0, 0),
            updated_at: chrono::NaiveDate::from_ymd_opt(2022, 10, 10)
                .expect("Failed to create date")
                .and_hms_opt(0, 0, 0),
            activation_state: None,
            activation_token: None,
            activation_token_expires_at: None,
            remember_me_token: None,
            remember_me_token_expires_at: None,
            reset_password_token: None,
            reset_password_token_expires_at: None,
            reset_password_email_sent_at: None,
            access_state: None,
            deleted_at: None,
            last_login_at: None,
            last_logout_at: None,
            last_activity_at: None,
            last_login_from_ip_address: None,
            language: None,
        }
    }
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct GenParameters {
    projects: Vec<Project>,
}

impl GenParameters {
    pub fn fill_database(self, connection: &mut PgConnection) -> Result<(), Error> {
        let mut projects = Vec::new();
        let mut members = Vec::new();
        let mut jobs = Vec::new();

        let mut organizations = HashMap::<i32, CoreOrganization>::new();
        let mut users = HashMap::<i32, User>::new();
        let mut next_member_id = 0;
        let mut next_job_id = 0;

        for (project_id, mut project) in self.projects.into_iter().enumerate() {
            let org_id = project.members[0].organization_id;
            for mut member in project.members.drain(..) {
                if !users.contains_key(&member.organization_id) {
                    let new_organization = CoreOrganization::generate_model(member.organization_id);
                    organizations.insert(member.organization_id, new_organization);
                }
                if !organizations.contains_key(&member.user_id) {
                    let new_user = User::generate_model(member.user_id);
                    users.insert(member.user_id, new_user);
                }
                for job in member.jobs.drain(..) {
                    jobs.push(job.generate_model(next_job_id, member.user_id, project_id as i32));
                    next_job_id += 1;
                }
                members.push(member.generate_model(next_member_id, project_id as i32));
                next_member_id += 1;
            }
            projects.push(project.generate_model(project_id as i32, org_id));
        }

        diesel::insert_into(crate::schema::core_projects::table)
            .values(&projects)
            .execute(connection)?;

        diesel::insert_into(crate::schema::core_members::table)
            .values(&members)
            .execute(connection)?;

        diesel::insert_into(crate::schema::jobstat_jobs::table)
            .values(&jobs)
            .execute(connection)?;

        let organizations: Vec<_> = organizations.values().collect();
        diesel::insert_into(crate::schema::core_organizations::table)
            .values(organizations)
            .execute(connection)?;

        let users: Vec<_> = users.values().collect();
        diesel::insert_into(crate::schema::users::table)
            .values(users)
            .execute(connection)?;

        Ok(())
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
            "__comment": "Student's practicum",
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
            activation_time: NaiveDateTime::parse_from_str(
                "01.05.2002 00:00:00",
                "%d.%m.%Y %H:%M:%S",
            )
            .expect("Failed to parse from str"),
            finish_time: NaiveDateTime::parse_from_str("01.07.2002 00:00:00", "%d.%m.%Y %H:%M:%S")
                .expect("Failed to parse from str"),
            estimated_finish_time: NaiveDateTime::parse_from_str(
                "01.06.2002 00:00:00",
                "%d.%m.%Y %H:%M:%S",
            )
            .expect("Failed to parse from str"),
            members: vec![sample_member],
        };
        assert_eq!(project, sample_project);
    }
}
