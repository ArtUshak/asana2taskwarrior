use std::collections::HashMap;

use chrono::{DateTime, NaiveDateTime, NaiveTime, TimeZone, Utc};
use uuid::Uuid;

use crate::{
    asana::{self},
    mapping::SectionPriorityMapping,
    taskwarrior::{self, get_depends_fields, Priority, UUID},
};

pub fn get_uuid_by_gid(uuids: &mut HashMap<String, Uuid>, gid: &str) -> Uuid {
    if let Some(uuid_value) = uuids.get(gid) {
        *uuid_value
    } else {
        let uuid = Uuid::new_v4();
        uuids.insert(gid.to_string(), uuid);
        uuid
    }
}

pub type ParentTaskData = (
    Uuid,
    Option<Priority>,
    Option<DateTime<Utc>>,
    Option<String>,
);

/// Convert Asana task and its subtasks to Taskwarrior, add output tasks to to `output_tasks`
pub fn convert_tasks_list(
    asana_tasks: Vec<asana::Task>,
    uuids: &mut HashMap<String, Uuid>,
    parent: &Option<ParentTaskData>,
    output_tasks: &mut Vec<taskwarrior::Task>,
    section_priority_mapping: &SectionPriorityMapping,
    children_to_dependencies: bool,
    append_sections_to_project: bool,
) -> Option<()> {
    for task in asana_tasks {
        let uuid = get_uuid_by_gid(uuids, &task.gid);

        let taskwarrior_task: taskwarrior::Task;

        if task.name.is_empty() {
            continue;
        }

        let membership = task.memberships.first(); // TODO

        let priority: Option<Priority>;
        if let Some((_, parent_priority, _, _)) = parent {
            priority = parent_priority.clone();
        } else if let Some(membership) = membership {
            priority = section_priority_mapping.get_mapping(&membership.section.name);
        } else {
            priority = None;
        }

        let project_name;
        if let Some((_, _, _, Some(parent_project_name))) = parent {
            project_name = Some(parent_project_name.clone());
        } else if append_sections_to_project {
            project_name = membership.map(|membership| {
                membership.project.name.clone() + ". " + &membership.section.name
            });
        } else {
            project_name = membership.map(|membership| membership.project.name.clone());
        }

        let parent_value = parent.as_ref().map(|parent| UUID::new(parent.0));
        let tags = Some(task.tags.iter().map(|tag| tag.name.clone()).collect());

        let due = task.due_on.map(|due_date| {
            Utc.from_utc_datetime(&NaiveDateTime::new(
                due_date,
                NaiveTime::from_hms_milli_opt(23, 59, 59, 999).unwrap(),
            ))
        });

        let completed_at: Option<DateTime<Utc>>;
        if let Some((_, _, Some(parent_completed_at), _)) = parent {
            completed_at = Some(*parent_completed_at);
        } else {
            completed_at = task.completed_at;
        }

        let asana_dependencies = task
            .dependencies
            .map(|dependencies| {
                dependencies
                    .iter()
                    .map(|dependency| get_uuid_by_gid(uuids, &dependency.gid))
                    .collect()
            })
            .unwrap_or_else(Vec::new);

        let mut dependencies_tmp: Vec<Uuid> = asana_dependencies;
        if children_to_dependencies {
            if let Some(subtasks) = &task.subtasks {
                for subtask in subtasks {
                    dependencies_tmp.push(get_uuid_by_gid(uuids, &subtask.gid));
                }
            }
        }
        let dependencies = if dependencies_tmp.is_empty() {
            None
        } else {
            Some(dependencies_tmp)
        };

        if let Some(completed_at) = completed_at {
            taskwarrior_task = taskwarrior::Task {
                status: taskwarrior::Status::Completed,
                uuid: UUID::new(uuid),
                entry: task.created_at,
                description: task.name,
                start: task.start_on,
                end: Some(completed_at),
                due,
                until: None,
                wait: None,
                modified: None,
                scheduled: None,
                recur: None,
                mask: None,
                imask: None,
                parent: parent_value,
                project: project_name.clone(),
                priority: priority.clone(),
                depends: dependencies.map(get_depends_fields),
                tags,
                annotation: None,
            };
        } else {
            taskwarrior_task = taskwarrior::Task {
                status: taskwarrior::Status::Pending,
                uuid: UUID::new(uuid),
                entry: task.created_at,
                description: task.name,
                start: task.start_on,
                end: None,
                due,
                until: None,
                wait: None,
                modified: None,
                scheduled: None,
                recur: None,
                mask: None,
                imask: None,
                parent: parent_value,
                project: project_name.clone(),
                priority: priority.clone(),
                depends: dependencies.map(get_depends_fields),
                tags,
                annotation: None,
            };
        }

        output_tasks.push(taskwarrior_task);

        if let Some(subtasks) = task.subtasks {
            convert_tasks_list(
                subtasks,
                uuids,
                &Some((uuid, priority, completed_at, project_name)),
                output_tasks,
                section_priority_mapping,
                children_to_dependencies,
                append_sections_to_project,
            )?;
        }
    }

    Some(())
}

/// Convert Asana tasks to Taskwarrior tasks.
pub fn convert_tasks(
    asana_tasks: Vec<asana::Task>,
    section_priority_mapping: &SectionPriorityMapping,
    children_to_dependencies: bool,
    append_sections_to_project: bool,
) -> Option<Vec<taskwarrior::Task>> {
    let mut uuids = HashMap::new();
    let mut output_tasks = Vec::new();

    convert_tasks_list(
        asana_tasks,
        &mut uuids,
        &None,
        &mut output_tasks,
        section_priority_mapping,
        children_to_dependencies,
        append_sections_to_project,
    )?;

    Some(output_tasks)
}
