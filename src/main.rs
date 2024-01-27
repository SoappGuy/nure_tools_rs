use color_eyre::Result;
use nure_tools::utils::Period;
fn main() -> Result<()> {
    color_eyre::install()?;

    let day_from = Period::day_from("2023-01-02");

    println!("{}", day_from);

    Ok(())
}
