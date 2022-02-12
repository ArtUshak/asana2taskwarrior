pub mod asana;
pub mod convert;
pub mod mapping;
pub mod taskwarrior;
mod test;

use clap::Parser;
use std::{fs::File, path::PathBuf};

use mapping::SectionPriorityMapping;

use crate::convert::convert_tasks;

const VERSION: &str = "0.1.0";

#[derive(Parser)]
#[clap(version = VERSION, author = "Artiom Khandamirov <t9max@yandex.ru>")]
struct CLIOptions {
    #[clap(short, long)]
    pub input_asana_file: PathBuf,
    #[clap(short, long)]
    pub output_taskwarrior_file: PathBuf,
    #[clap(long)]
    pub section_priority_mapping_file: Option<PathBuf>,
    #[clap(long)]
    pub children_to_dependencies: bool,
    #[clap(long)]
    pub append_sections_to_project: bool,
}

fn main() {
    let opts: CLIOptions = CLIOptions::parse();

    let input_asana_data: asana::Exported;
    {
        let input_asana_file = File::open(opts.input_asana_file).unwrap();
        input_asana_data = serde_json::from_reader(input_asana_file).unwrap();
    }

    let section_priority_mapping: SectionPriorityMapping;
    if let Some(section_priority_mapping_file_path) = opts.section_priority_mapping_file {
        let section_priority_mapping_file = File::open(section_priority_mapping_file_path).unwrap();
        section_priority_mapping = serde_json::from_reader(section_priority_mapping_file).unwrap();
    } else {
        section_priority_mapping = SectionPriorityMapping::default();
    }

    let output_taskwarrior_data = convert_tasks(
        input_asana_data.data,
        &section_priority_mapping,
        opts.children_to_dependencies,
        opts.append_sections_to_project,
    )
    .unwrap();

    {
        let output_taskwarrior_file = File::create(opts.output_taskwarrior_file).unwrap();
        serde_json::to_writer(output_taskwarrior_file, &output_taskwarrior_data).unwrap();
    }
}
