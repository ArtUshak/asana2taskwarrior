use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::taskwarrior::Priority;

/// Mapping from Asana section name to Taskwarrior priority of tasks.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct SectionPriorityMapping {
    pub default_mapping: Option<Priority>,
    pub mapping: HashMap<String, Priority>,
}

impl SectionPriorityMapping {
    pub fn get_mapping(&self, section_name: &str) -> Option<Priority> {
        match self.mapping.get(section_name) {
            Some(priority) => Some(priority.clone()),
            None => self.default_mapping.clone(),
        }
    }
}
