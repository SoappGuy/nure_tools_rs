use color_eyre::Result;
use nure_tools::{
    groups::{find_group, Group},
    schedule::{get_schedule, Lecture, Request},
    utils::Period,
};
fn main() -> Result<()> {
    color_eyre::install()?;

    let groups_response: Vec<Group> = find_group("пзпі-23-2")?;

    // println!(
    //     "{:#?}",
    //     Period::from_string("2024-01-10 00:00:00", "2024-01-10 23:59:59")
    // );
    for group in groups_response {
        let schedule_request_bygroup: Request = Request::Group(group);
        let schedule_response: Vec<Lecture> = get_schedule(
            schedule_request_bygroup,
            Period::from_string("2023-12-19 00:00:00", "2023-12-19 23:59:59"),
        )?;
        print!("{:#?}", schedule_response);
    }

    Ok(())
}
