use anyhow::Error;
use nure_tools::{
    groups::{find_group, Group},
    schedule::{get_schedule, Lecture, Request},
    utils::Period,
};

fn main() -> Result<(), Error> {
    let groups_response: Vec<Group> = find_group("пзпі-23-2")?;
    let _a = 2;

    for group in groups_response {
        let schedule_request_bygroup: Request = Request::Group(group);
        let schedule_response: Vec<Lecture> = get_schedule(
            schedule_request_bygroup,
            Period::from_string("2024-01-02", "2024-01-03")?,
        )?;
        println!("{:#?}", schedule_response);
    }
    Ok::<(), Error>(())
}
