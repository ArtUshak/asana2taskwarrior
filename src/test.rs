#[cfg(test)]
mod tests {
    use crate::{asana, convert::convert_tasks, mapping::SectionPriorityMapping, taskwarrior};
    use std::{collections::HashMap, fs::File, path::Path};

    fn assert_tasks_equal(task1: &taskwarrior::Task, task2: &taskwarrior::Task) {
        assert_eq!(task1.status, task2.status);
        assert_eq!(task1.entry, task2.entry);
        assert_eq!(task1.description, task2.description);
        assert_eq!(task1.start, task2.start);
        assert_eq!(task1.end, task2.end);
        assert_eq!(task1.due, task2.due);
        assert_eq!(task1.until, task2.until);
        assert_eq!(task1.wait, task2.wait);
        assert_eq!(task1.modified, task2.modified);
        assert_eq!(task1.scheduled, task2.scheduled);
        assert_eq!(task1.recur, task2.recur);
        assert_eq!(task1.mask, task2.mask);
        assert_eq!(task1.imask, task2.imask);
        assert_eq!(task1.project, task2.project);
        assert_eq!(task1.priority, task2.priority);
        assert_eq!(task1.depends, task2.depends);
        assert_eq!(task1.tags, task2.tags);
    }

    #[test]
    fn test1() {
        let input_file_path = Path::new("testfiles/input/1194733031423185.json");
        let correct_output_file_path = Path::new("testfiles/correct_output/1194733031423185.json");

        let input_asana_data: asana::Exported;
        {
            let input_asana_file = File::open(input_file_path).unwrap();
            input_asana_data = serde_json::from_reader(input_asana_file).unwrap();
        }

        let section_priority_mapping = SectionPriorityMapping {
            default_mapping: None,
            mapping: HashMap::new(),
        };

        let output_taskwarrior_data =
            convert_tasks(input_asana_data.data, &section_priority_mapping, true, true).unwrap();

        let correct_output_data: Vec<taskwarrior::Task>;
        {
            let correct_output_file = File::open(correct_output_file_path).unwrap();
            correct_output_data = serde_json::from_reader(correct_output_file).unwrap();
        }

        assert_eq!(output_taskwarrior_data.len(), correct_output_data.len());
        for (output_task, correct_task) in output_taskwarrior_data
            .iter()
            .zip(correct_output_data.iter())
        {
            assert_tasks_equal(output_task, correct_task);
        }

        // TODO!
    }
}
