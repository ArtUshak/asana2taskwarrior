//! Asana export format types
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Exported {
    pub data: Vec<Task>,
}

/// Asana task data, according to https://developers.asana.com/docs/task
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub gid: String,
    pub resource_type: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_status: Option<String>,
    pub assignee_status: String,
    pub completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_by: Option<UserReference>,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<TaskReference>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependents: Option<Vec<TaskReference>>,
    pub due_at: Option<DateTime<Utc>>,
    pub due_on: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<TaskExternal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rendered_as_separator: Option<bool>,
    pub liked: bool,
    pub likes: Vec<Like>,
    pub projects: Vec<ProjectReference>,
    pub memberships: Vec<TaskMembership>,
    pub modified_at: DateTime<Utc>,
    pub notes: String,
    pub num_likes: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_subtasks: Option<u64>,
    pub resource_subtype: String,
    pub start_on: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<UserReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_section: Option<SectionReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
    pub followers: Vec<UserReference>,
    pub parent: Option<TaskReference>,
    pub tags: Vec<TagReference>,
    pub workspace: WorkspaceReference,
    pub subtasks: Option<Vec<Task>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TaskMembership {
    pub project: ProjectReference,
    pub section: SectionReference,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TaskExternal {
    pub gid: String,
    pub data: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TaskReference {
    pub gid: String,
    pub resource_type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Like {
    pub gid: String,
    pub user: UserReference,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Reference {
    pub gid: String,
    pub resource_type: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomField {
    pub gid: String,
    pub resource_type: String,
    pub created_by: UserReference,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_label_position: Option<CustomLabelPosition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub display_value: String,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_options: Option<Vec<EnumOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_value: Option<EnumOption>,
    pub format: CustomFieldFormat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_notifications_enabled: Option<bool>,
    pub is_global_to_workspace: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_enum_values: Option<Vec<EnumOption>>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<u8>,
    pub resource_subtype: String,
    pub text_value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CustomLabelPosition {
    #[serde(rename = "prefix")]
    Prefix,
    #[serde(rename = "prefix")]
    Suffix,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CustomFieldFormat {
    #[serde(rename = "currency")]
    Currency,
    #[serde(rename = "identifier")]
    Identifier,
    #[serde(rename = "percentage")]
    Percentage,
    #[serde(rename = "custom")]
    Custom,
    #[serde(rename = "none")]
    None,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EnumOption {
    pub gid: String,
    pub resource_type: String,
    pub color: String,
    pub enabled: bool,
    pub name: String,
}

pub type UserReference = Reference;
pub type ProjectReference = Reference;
pub type SectionReference = Reference;
pub type TagReference = Reference;
pub type WorkspaceReference = Reference;
