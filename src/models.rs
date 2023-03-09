// Generated by diesel_ext
#![allow(unused)]
#![allow(clippy::all)]

use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use crate::schema::jobstat_jobs;


#[derive(Queryable, Debug)]
pub struct ActiveStorageAttachment {
    pub id: i64,
    pub name: String,
    pub record_type: String,
    pub record_id: i64,
    pub blob_id: i64,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct ActiveStorageBlob {
    pub id: i64,
    pub key: String,
    pub filename: String,
    pub content_type: Option<String>,
    pub metadata: Option<String>,
    pub byte_size: i64,
    pub checksum: String,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct AnnouncementRecipient {
    pub id: i32,
    pub user_id: Option<i32>,
    pub announcement_id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct Announcement {
    pub id: i32,
    pub title_ru: Option<String>,
    pub reply_to: Option<String>,
    pub body_ru: Option<String>,
    pub attachment: Option<String>,
    pub is_special: Option<bool>,
    pub state: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_by_id: Option<i32>,
    pub title_en: Option<String>,
    pub body_en: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct ApiAccessKey {
    pub id: i32,
    pub key: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct ApiAccessKeysExport {
    pub id: i32,
    pub access_key_id: Option<i32>,
    pub export_id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct ApiExport {
    pub id: i32,
    pub title: Option<String>,
    pub request: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub text: Option<String>,
    pub safe: Option<bool>,
}

#[derive(Queryable, Debug)]
pub struct ApiKeyParameter {
    pub id: i32,
    pub name: Option<String>,
    pub default: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
#[diesel(primary_key(key))]
pub struct ArInternalMetadata {
    pub key: String,
    pub value: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct CategoryValue {
    pub id: i32,
    pub options_category_id: Option<i32>,
    pub value_ru: Option<String>,
    pub value_en: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct CloudComputingAccesse {
    pub id: i64,
    pub finish_date: Option<NaiveDate>,
    pub user_id: Option<i64>,
    pub allowed_by_id: Option<i64>,
    pub for_type: Option<String>,
    pub for_id: Option<i64>,
    pub state: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct CloudComputingApiLog {
    pub id: i64,
    pub virtual_machine_id: Option<i64>,
    pub item_id: Option<i64>,
    pub log: Option<String>,
    pub action: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct CloudComputingCondition {
    pub id: i64,
    pub from_type: Option<String>,
    pub from_id: Option<i64>,
    pub to_type: Option<String>,
    pub to_id: Option<i64>,
    pub from_multiplicity: Option<String>,
    pub to_multiplicity: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct CloudComputingItemLink {
    pub id: i64,
    pub from_id: Option<i64>,
    pub to_id: Option<i64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct CloudComputingItem {
    pub id: i64,
    pub template_id: Option<i64>,
    pub item_id: Option<i64>,
    pub holder_type: Option<String>,
    pub holder_id: Option<i64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct CloudComputingRequest {
    pub id: i64,
    pub comment: Option<String>,
    pub admin_comment: Option<String>,
    pub finish_date: Option<NaiveDate>,
    pub created_by_id: Option<i64>,
    pub access_id: Option<i64>,
    pub for_type: Option<String>,
    pub for_id: Option<i64>,
    pub status: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct CloudComputingResourceItem {
    pub id: i64,
    pub resource_id: Option<i64>,
    pub item_id: Option<i64>,
    pub value: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct CloudComputingResourceKind {
    pub id: i64,
    pub name_ru: Option<String>,
    pub name_en: Option<String>,
    pub description_en: Option<String>,
    pub description_ru: Option<String>,
    pub help_en: Option<String>,
    pub help_ru: Option<String>,
    pub measurement_ru: Option<String>,
    pub measurement_en: Option<String>,
    pub identity: Option<String>,
    pub content_type: Option<i32>,
    pub template_kind_id: Option<i64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct CloudComputingResource {
    pub id: i64,
    pub resource_kind_id: Option<i64>,
    pub template_id: Option<i64>,
    pub value: Option<String>,
    pub editable: Option<bool>,
    pub min: Option<BigDecimal>,
    pub max: Option<BigDecimal>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct CloudComputingTemplateKind {
    pub id: i64,
    pub name_ru: Option<String>,
    pub name_en: Option<String>,
    pub description_ru: Option<String>,
    pub description_en: Option<String>,
    pub cloud_class: Option<String>,
    pub parent_id: Option<i32>,
    pub lft: i32,
    pub rgt: i32,
    pub depth: i32,
    pub children_count: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct CloudComputingTemplate {
    pub id: i64,
    pub template_kind_id: Option<i64>,
    pub name_ru: Option<String>,
    pub name_en: Option<String>,
    pub description_ru: Option<String>,
    pub description_en: Option<String>,
    pub identity: Option<i32>,
    pub new_requests: Option<bool>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct CloudComputingVirtualMachine {
    pub id: i64,
    pub identity: Option<i32>,
    pub item_id: Option<i64>,
    pub inner_address: Option<String>,
    pub internet_address: Option<String>,
    pub state: Option<String>,
    pub lcm_state: Option<String>,
    pub last_info: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct CommentsComment {
    pub id: i32,
    pub text: Option<String>,
    pub attachable_id: i32,
    pub attachable_type: String,
    pub user_id: i32,
    pub context_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct CommentsContextGroup {
    pub id: i32,
    pub context_id: i32,
    pub group_id: i32,
    pub type_ab: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct CommentsContext {
    pub id: i32,
    pub name_ru: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name_en: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct CommentsFileAttachment {
    pub id: i32,
    pub file: Option<String>,
    pub description: Option<String>,
    pub attachable_id: i32,
    pub attachable_type: String,
    pub user_id: i32,
    pub context_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct CommentsGroupClasse {
    pub id: i32,
    pub class_name: Option<String>,
    pub obj_id: Option<i32>,
    pub group_id: Option<i32>,
    pub allow: bool,
    pub type_ab: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct CommentsTagging {
    pub id: i32,
    pub tag_id: Option<i32>,
    pub attachable_id: i32,
    pub attachable_type: String,
    pub user_id: Option<i32>,
    pub context_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct CommentsTag {
    pub id: i32,
    pub name: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct CoreAccessField {
    pub id: i32,
    pub access_id: Option<i32>,
    pub quota: Option<i32>,
    pub used: Option<i32>,
    pub quota_kind_id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct CoreAccesse {
    pub id: i32,
    pub project_id: i32,
    pub cluster_id: i32,
    pub state: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub project_group_name: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct CoreBotLink {
    pub id: i64,
    pub user_id: Option<i64>,
    pub token: Option<String>,
    pub active: Option<bool>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct CoreCity {
    pub id: i32,
    pub country_id: Option<i32>,
    pub title_ru: Option<String>,
    pub title_en: Option<String>,
    pub checked: Option<bool>,
}

#[derive(Queryable, Debug)]
pub struct CoreClusterLog {
    pub id: i32,
    pub cluster_id: i32,
    pub message: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub project_id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct CoreClusterQuota {
    pub id: i32,
    pub cluster_id: i32,
    pub value: Option<i32>,
    pub quota_kind_id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct CoreCluster {
    pub id: i32,
    pub name_ru: String,
    pub host: String,
    pub description: Option<String>,
    pub public_key: Option<String>,
    pub private_key: Option<String>,
    pub admin_login: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub available_for_work: Option<bool>,
    pub name_en: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct CoreCountry {
    pub id: i32,
    pub title_ru: Option<String>,
    pub title_en: Option<String>,
    pub checked: Option<bool>,
}

#[derive(Queryable, Debug)]
pub struct CoreCredential {
    pub id: i32,
    pub user_id: i32,
    pub state: Option<String>,
    pub name: String,
    pub public_key: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug)]
pub struct CoreCriticalTechnology {
    pub id: i32,
    pub name_ru: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub name_en: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct CoreCriticalTechnologiesPerProject {
    pub id: i32,
    pub critical_technology_id: Option<i32>,
    pub project_id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct CoreDepartmentMerger {
    pub id: i32,
    pub source_department_id: Option<i32>,
    pub to_organization_id: Option<i32>,
    pub to_department_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct CoreDirectionOfScience {
    pub id: i32,
    pub name_ru: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub name_en: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct CoreDirectionOfSciencesPerProject {
    pub id: i32,
    pub direction_of_science_id: Option<i32>,
    pub project_id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct CoreEmploymentPositionField {
    pub id: i32,
    pub employment_position_name_id: Option<i32>,
    pub name_ru: Option<String>,
    pub name_en: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct CoreEmploymentPositionName {
    pub id: i32,
    pub name_ru: Option<String>,
    pub autocomplete: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub name_en: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct CoreEmploymentPosition {
    pub id: i32,
    pub employment_id: Option<i32>,
    pub name: Option<String>,
    pub value: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub employment_position_name_id: Option<i32>,
    pub field_id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct CoreEmployment {
    pub id: i32,
    pub user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub primary: Option<bool>,
    pub state: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub organization_department_id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct CoreGroupOfResearchArea {
    pub id: i64,
    pub name_en: Option<String>,
    pub name_ru: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct CoreMember {
    pub id: i32,
    pub user_id: i32,
    pub project_id: i32,
    pub owner: Option<bool>,
    pub login: Option<String>,
    pub project_access_state: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub organization_id: Option<i32>,
    pub organization_department_id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct CoreNoticeShowOption {
    pub id: i64,
    pub user_id: Option<i64>,
    pub notice_id: Option<i64>,
    pub hidden: bool,
    pub resolved: bool,
    pub answer: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct CoreNotice {
    pub id: i32,
    pub sourceable_type: Option<String>,
    pub sourceable_id: Option<i32>,
    pub linkable_type: Option<String>,
    pub linkable_id: Option<i32>,
    pub message: Option<String>,
    pub count: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub category: Option<i32>,
    pub kind: Option<String>,
    pub show_from: Option<NaiveDateTime>,
    pub show_till: Option<NaiveDateTime>,
    pub active: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct CoreOrganizationDepartment {
    pub id: i32,
    pub organization_id: Option<i32>,
    pub name: Option<String>,
    pub checked: Option<bool>,
}

#[derive(Queryable, Debug)]
pub struct CoreOrganizationKind {
    pub id: i32,
    pub name_ru: Option<String>,
    pub departments_required: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub name_en: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct CoreOrganization {
    pub id: i32,
    pub name: Option<String>,
    pub abbreviation: Option<String>,
    pub kind_id: Option<i32>,
    pub country_id: Option<i32>,
    pub city_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub checked: Option<bool>,
}

#[derive(Queryable, Debug)]
pub struct CorePartition {
    pub id: i32,
    pub name: Option<String>,
    pub cluster_id: Option<i32>,
    pub resources: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct CoreProjectCard {
    pub id: i32,
    pub project_id: Option<i32>,
    pub name: Option<String>,
    pub en_name: Option<String>,
    pub driver: Option<String>,
    pub en_driver: Option<String>,
    pub strategy: Option<String>,
    pub en_strategy: Option<String>,
    pub objective: Option<String>,
    pub en_objective: Option<String>,
    pub impact: Option<String>,
    pub en_impact: Option<String>,
    pub usage: Option<String>,
    pub en_usage: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug)]
pub struct CoreProjectInvitation {
    pub id: i32,
    pub project_id: i32,
    pub user_fio: String,
    pub user_email: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub language: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct CoreProjectKind {
    pub id: i32,
    pub name_ru: Option<String>,
    pub name_en: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct CoreProject {
    pub id: i32,
    pub title: String,
    pub state: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub organization_id: Option<i32>,
    pub organization_department_id: Option<i32>,
    pub kind_id: Option<i32>,
    pub first_activation_at: Option<NaiveDateTime>,
    pub finished_at: Option<NaiveDateTime>,
    pub estimated_finish_date: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug)]
pub struct CoreQuotaKind {
    pub id: i32,
    pub name_ru: Option<String>,
    pub measurement_ru: Option<String>,
    pub name_en: Option<String>,
    pub measurement_en: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct CoreRequestField {
    pub id: i32,
    pub request_id: i32,
    pub value: Option<i32>,
    pub quota_kind_id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct CoreRequest {
    pub id: i32,
    pub project_id: i32,
    pub cluster_id: i32,
    pub state: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub cpu_hours: Option<i32>,
    pub gpu_hours: Option<i32>,
    pub hdd_size: Option<i32>,
    pub group_name: Option<String>,
    pub creator_id: Option<i32>,
    pub comment: Option<String>,
    pub reason: Option<String>,
    pub changed_by_id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct CoreResearchArea {
    pub id: i32,
    pub name_ru: Option<String>,
    pub old_group: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub name_en: Option<String>,
    pub group_id: Option<i64>,
}

#[derive(Queryable, Debug)]
pub struct CoreResearchAreasPerProject {
    pub id: i32,
    pub research_area_id: Option<i32>,
    pub project_id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct CoreSurety {
    pub id: i32,
    pub project_id: Option<i32>,
    pub state: Option<String>,
    pub comment: Option<String>,
    pub boss_full_name: Option<String>,
    pub boss_position: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub document: Option<String>,
    pub author_id: Option<i32>,
    pub reason: Option<String>,
    pub changed_by_id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct CoreSuretyMember {
    pub id: i32,
    pub user_id: Option<i32>,
    pub surety_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub organization_department_id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct CoreSuretyScan {
    pub id: i32,
    pub surety_id: Option<i32>,
    pub image: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct DelayedJob {
    pub id: i32,
    pub priority: i32,
    pub attempts: i32,
    pub handler: String,
    pub last_error: Option<String>,
    pub run_at: Option<NaiveDateTime>,
    pub locked_at: Option<NaiveDateTime>,
    pub failed_at: Option<NaiveDateTime>,
    pub locked_by: Option<String>,
    pub queue: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug)]
pub struct FaceMenuItemPref {
    pub id: i64,
    pub position: Option<i32>,
    pub menu: Option<String>,
    pub key: Option<String>,
    pub user_id: Option<i64>,
    pub admin: Option<bool>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct FaceUsersMenu {
    pub id: i64,
    pub menu: Option<String>,
    pub user_id: Option<i64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct Group {
    pub id: i32,
    pub name: Option<String>,
    pub weight: Option<i32>,
    pub system: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug)]
pub struct HardwareItem {
    pub id: i32,
    pub name_ru: Option<String>,
    pub name_en: Option<String>,
    pub description_ru: Option<String>,
    pub description_en: Option<String>,
    pub lock_version: Option<i32>,
    pub kind_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct HardwareItemsState {
    pub id: i32,
    pub item_id: Option<i32>,
    pub state_id: Option<i32>,
    pub reason_en: Option<String>,
    pub reason_ru: Option<String>,
    pub description_en: Option<String>,
    pub description_ru: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct HardwareKind {
    pub id: i32,
    pub name_ru: Option<String>,
    pub name_en: Option<String>,
    pub description_ru: Option<String>,
    pub description_en: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct HardwareState {
    pub id: i32,
    pub name_ru: Option<String>,
    pub name_en: Option<String>,
    pub description_ru: Option<String>,
    pub description_en: Option<String>,
    pub lock_version: Option<i32>,
    pub kind_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct HardwareStatesLink {
    pub id: i32,
    pub from_id: Option<i32>,
    pub to_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct JobstatDataType {
    pub id: i32,
    pub name: Option<String>,
    pub type_: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct JobstatDigestFloatData {
    pub id: i32,
    pub name: Option<String>,
    pub job_id: Option<i64>,
    pub value: Option<f64>,
    pub time: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct JobstatDigestStringData {
    pub id: i32,
    pub name: Option<String>,
    pub job_id: Option<i64>,
    pub value: Option<String>,
    pub time: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct JobstatFloatData {
    pub id: i32,
    pub name: Option<String>,
    pub job_id: Option<i64>,
    pub value: Option<f64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct JobstatJobMailFilter {
    pub id: i32,
    pub condition: Option<String>,
    pub user_id: Option<i32>,
}


#[derive(Queryable, Debug, Insertable)]
pub struct JobstatJob {
    pub id: i32,
    pub cluster: Option<String>,
    pub drms_job_id: Option<i64>,
    pub drms_task_id: Option<i64>,
    pub login: Option<String>,
    pub partition: Option<String>,
    pub submit_time: Option<NaiveDateTime>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub timelimit: Option<i64>,
    pub command: Option<String>,
    pub state: Option<String>,
    pub num_cores: Option<i64>,
    pub num_nodes: Option<i64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub nodelist: Option<String>,
    pub initiator_id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct JobstatStringData {
    pub id: i32,
    pub name: Option<String>,
    pub job_id: Option<i64>,
    pub value: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct Options {
    pub id: i32,
    pub owner_id: Option<i32>,
    pub owner_type: Option<String>,
    pub name_ru: Option<String>,
    pub name_en: Option<String>,
    pub value_ru: Option<String>,
    pub value_en: Option<String>,
    pub category_value_id: Option<i32>,
    pub options_category_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub admin: Option<bool>,
}

#[derive(Queryable, Debug)]
pub struct OptionsCategory {
    pub id: i32,
    pub name_ru: Option<String>,
    pub name_en: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct PackAccesse {
    pub id: i32,
    pub version_id: Option<i32>,
    pub who_id: Option<i32>,
    pub who_type: Option<String>,
    pub status: Option<String>,
    pub created_by_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub end_lic: Option<NaiveDate>,
    pub new_end_lic: Option<NaiveDate>,
    pub allowed_by_id: Option<i32>,
    pub lock_version: i32,
    pub new_end_lic_forever: Option<bool>,
    pub to_type: Option<String>,
    pub to_id: Option<i64>,
}

#[derive(Queryable, Debug)]
pub struct PackCategoryValue {
    pub id: i32,
    pub options_category_id: Option<i32>,
    pub value_ru: Option<String>,
    pub value_en: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct PackClusterver {
    pub id: i32,
    pub core_cluster_id: Option<i32>,
    pub version_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub active: Option<bool>,
    pub path: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct PackOptionsCategory {
    pub id: i32,
    pub category_ru: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub category_en: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct PackPackage {
    pub id: i32,
    pub name_ru: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub description_ru: Option<String>,
    pub deleted: bool,
    pub description_en: Option<String>,
    pub name_en: Option<String>,
    pub accesses_to_package: Option<bool>,
}

#[derive(Queryable, Debug)]
pub struct PackVersionOption {
    pub id: i32,
    pub version_id: Option<i32>,
    pub name_ru: Option<String>,
    pub value_ru: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name_en: Option<String>,
    pub value_en: Option<String>,
    pub category_value_id: Option<i32>,
    pub options_category_id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct PackVersion {
    pub id: i32,
    pub name_ru: Option<String>,
    pub description_ru: Option<String>,
    pub package_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub cost: Option<i32>,
    pub end_lic: Option<NaiveDate>,
    pub state: Option<String>,
    pub lock_col: i32,
    pub deleted: bool,
    pub service: bool,
    pub delete_on_expire: bool,
    pub ticket_id: Option<i32>,
    pub description_en: Option<String>,
    pub name_en: Option<String>,
    pub ticket_created: Option<bool>,
}

#[derive(Queryable, Debug)]
pub struct Permission {
    pub id: i32,
    pub action: Option<String>,
    pub subject_class: Option<String>,
    pub group_id: Option<i32>,
    pub available: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub subject_id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct Profile {
    pub id: i32,
    pub user_id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub middle_name: Option<String>,
    pub about: Option<String>,
    pub receive_info_mails: Option<bool>,
    pub receive_special_mails: Option<bool>,
}

#[derive(Queryable, Debug)]
#[diesel(primary_key(version))]
pub struct SchemaMigration {
    pub version: String,
}

#[derive(Queryable, Debug)]
pub struct SessionsManager {
    pub id: i64,
    pub user_id: Option<i64>,
    pub session_id: Option<i64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct SessionsProjectsInSession {
    pub id: i32,
    pub session_id: Option<i32>,
    pub project_id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct SessionsReportMaterial {
    pub id: i32,
    pub materials: Option<String>,
    pub materials_content_type: Option<String>,
    pub materials_file_size: Option<i32>,
    pub report_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct SessionsReportReply {
    pub id: i32,
    pub report_id: Option<i32>,
    pub user_id: Option<i32>,
    pub message: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug)]
pub struct SessionsReportSubmitDenialReason {
    pub id: i32,
    pub name_ru: Option<String>,
    pub name_en: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct SessionsReport {
    pub id: i32,
    pub session_id: Option<i32>,
    pub project_id: Option<i32>,
    pub author_id: Option<i32>,
    pub expert_id: Option<i32>,
    pub state: Option<String>,
    pub materials: Option<String>,
    pub materials_file_name: Option<String>,
    pub materials_content_type: Option<String>,
    pub materials_file_size: Option<i32>,
    pub materials_updated_at: Option<NaiveDateTime>,
    pub illustration_points: Option<i32>,
    pub summary_points: Option<i32>,
    pub statement_points: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub submit_denial_reason_id: Option<i32>,
    pub submit_denial_description: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct SessionsSession {
    pub id: i32,
    pub state: Option<String>,
    pub description_ru: Option<String>,
    pub motivation_ru: Option<String>,
    pub started_at: Option<NaiveDateTime>,
    pub ended_at: Option<NaiveDateTime>,
    pub receiving_to: Option<NaiveDateTime>,
    pub description_en: Option<String>,
    pub motivation_en: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct SessionsStat {
    pub id: i32,
    pub session_id: Option<i32>,
    pub survey_field_id: Option<i32>,
    pub group_by: Option<String>,
    pub weight: Option<i32>,
    pub organization_id: Option<i32>,
    pub cache: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct SessionsSurveyField {
    pub id: i32,
    pub survey_id: Option<i32>,
    pub kind: Option<String>,
    pub collection: Option<String>,
    pub max_values: Option<i32>,
    pub weight: Option<i32>,
    pub name_ru: Option<String>,
    pub required: Option<bool>,
    pub entity: Option<String>,
    pub strict_collection: Option<bool>,
    pub hint_ru: Option<String>,
    pub reference_type: Option<String>,
    pub regexp: Option<String>,
    pub hint_en: Option<String>,
    pub name_en: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct SessionsSurveyKind {
    pub id: i32,
    pub name: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct SessionsSurveyValue {
    pub id: i32,
    pub value: Option<String>,
    pub survey_field_id: Option<i32>,
    pub user_id: Option<i32>,
    pub user_survey_id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct SessionsSurvey {
    pub id: i32,
    pub session_id: Option<i32>,
    pub kind_id: Option<i32>,
    pub name_ru: Option<String>,
    pub only_for_project_owners: Option<bool>,
    pub name_en: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct SessionsUserSurvey {
    pub id: i32,
    pub user_id: Option<i32>,
    pub session_id: Option<i32>,
    pub survey_id: Option<i32>,
    pub project_id: Option<i32>,
    pub state: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug)]
pub struct StatisticsOrganizationStat {
    pub id: i32,
    pub kind: Option<String>,
    pub data: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug)]
pub struct StatisticsProjectStat {
    pub id: i32,
    pub kind: Option<String>,
    pub data: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug)]
pub struct StatisticsSessionStat {
    pub id: i32,
    pub kind: Option<String>,
    pub data: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug)]
pub struct StatisticsUserStat {
    pub id: i32,
    pub kind: Option<String>,
    pub data: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug)]
pub struct SupportFieldOption {
    pub id: i64,
    pub field_id: Option<i64>,
    pub name_ru: Option<String>,
    pub name_en: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct SupportFieldValue {
    pub id: i32,
    pub field_id: Option<i32>,
    pub ticket_id: Option<i32>,
    pub value: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub topics_field_id: Option<i64>,
}

#[derive(Queryable, Debug)]
pub struct SupportField {
    pub id: i32,
    pub name_ru: Option<String>,
    pub hint_ru: Option<String>,
    pub required: Option<bool>,
    pub contains_source_code: Option<bool>,
    pub url: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub name_en: Option<String>,
    pub hint_en: Option<String>,
    pub model_collection: Option<String>,
    pub kind: Option<i32>,
    pub search: Option<bool>,
}

#[derive(Queryable, Debug)]
pub struct SupportReply {
    pub id: i32,
    pub author_id: Option<i32>,
    pub ticket_id: Option<i32>,
    pub message: Option<String>,
    pub attachment: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub attachment_file_name: Option<String>,
    pub attachment_content_type: Option<String>,
    pub attachment_file_size: Option<i32>,
    pub attachment_updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug)]
pub struct SupportReplyTemplate {
    pub id: i32,
    pub subject_ru: Option<String>,
    pub message_ru: Option<String>,
    pub subject_en: Option<String>,
    pub message_en: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct SupportTag {
    pub id: i32,
    pub name_ru: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub name_en: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct SupportTicket {
    pub id: i32,
    pub topic_id: Option<i32>,
    pub project_id: Option<i32>,
    pub cluster_id: Option<i32>,
    pub surety_id: Option<i32>,
    pub reporter_id: Option<i32>,
    pub subject: Option<String>,
    pub message: Option<String>,
    pub state: Option<String>,
    pub url: Option<String>,
    pub attachment: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub responsible_id: Option<i32>,
    pub attachment_file_name: Option<String>,
    pub attachment_content_type: Option<String>,
    pub attachment_file_size: Option<i32>,
    pub attachment_updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug)]
pub struct SupportTicketsSubscriber {
    pub id: i32,
    pub ticket_id: Option<i32>,
    pub user_id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct SupportTicketsTag {
    pub id: i32,
    pub ticket_id: Option<i32>,
    pub tag_id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct SupportTopic {
    pub id: i32,
    pub name_ru: Option<String>,
    pub parent_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub name_en: Option<String>,
    pub template_en: Option<String>,
    pub template_ru: Option<String>,
    pub visible_on_create: Option<bool>,
}

#[derive(Queryable, Debug)]
pub struct SupportTopicsField {
    pub id: i32,
    pub topic_id: Option<i32>,
    pub field_id: Option<i32>,
    pub required: Option<bool>,
}

#[derive(Queryable, Debug)]
pub struct SupportTopicsTag {
    pub id: i32,
    pub topic_id: Option<i32>,
    pub tag_id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct SupportUserTopic {
    pub id: i32,
    pub user_id: Option<i32>,
    pub topic_id: Option<i32>,
    pub required: Option<bool>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct UserGroup {
    pub id: i32,
    pub user_id: Option<i32>,
    pub group_id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub crypted_password: Option<String>,
    pub salt: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub activation_state: Option<String>,
    pub activation_token: Option<String>,
    pub activation_token_expires_at: Option<NaiveDateTime>,
    pub remember_me_token: Option<String>,
    pub remember_me_token_expires_at: Option<NaiveDateTime>,
    pub reset_password_token: Option<String>,
    pub reset_password_token_expires_at: Option<NaiveDateTime>,
    pub reset_password_email_sent_at: Option<NaiveDateTime>,
    pub access_state: Option<String>,
    pub deleted_at: Option<NaiveDateTime>,
    pub last_login_at: Option<NaiveDateTime>,
    pub last_logout_at: Option<NaiveDateTime>,
    pub last_activity_at: Option<NaiveDateTime>,
    pub last_login_from_ip_address: Option<String>,
    pub language: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct Version {
    pub id: i32,
    pub item_type: String,
    pub item_id: i64,
    pub event: String,
    pub whodunnit: Option<i32>,
    pub object: Option<String>,
    pub session_id: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub object_changes: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct WikiPage {
    pub id: i32,
    pub name_ru: Option<String>,
    pub content_ru: Option<String>,
    pub url: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub name_en: Option<String>,
    pub content_en: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct WikiplusImage {
    pub id: i32,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct WikiplusPage {
    pub id: i32,
    pub name_ru: Option<String>,
    pub content_ru: Option<String>,
    pub url: Option<String>,
    pub show_all: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub name_en: Option<String>,
    pub content_en: Option<String>,
    pub sortid: Option<BigDecimal>,
    pub mainpage_id: Option<i32>,
    pub image: Option<String>,
}
