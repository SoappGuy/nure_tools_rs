use color_eyre::Result;
use nure_tools::teachers::{find_exect_teacher, Teacher};

fn main() -> Result<()> {
    color_eyre::install()?;

    let teacher: Teacher = find_exect_teacher("Терещенко Г. Ю.")?;
    println!("teacher: {:#?}", teacher);

    Ok(())
}
