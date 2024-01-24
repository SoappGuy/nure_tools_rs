use color_eyre::Result;
use nure_tools::utils::Period;

fn main() -> Result<()> {
    color_eyre::install()?;

    let start_time: i64 = 1704146400;
    let end_time: i64 = 1704232800;

    let period: Period = Period::from_timestamp(start_time, end_time);

    println!("Period: {:#?}", period);

    Ok(())
}
