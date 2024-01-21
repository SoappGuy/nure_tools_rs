use color_eyre::Result;
use nure_tools::lecture_rooms::{find_lecture_room, LectureRoom};

fn main() -> Result<()> {
    color_eyre::install()?;

    let lecture_room: Vec<LectureRoom> = find_lecture_room("і")?;
    println!("lecture_rooms: {:#?}\n", lecture_room);

    let lecture_room: Vec<LectureRoom> = find_lecture_room("філія")?;
    println!("lecture_rooms: {:#?}\n", lecture_room);

    Ok(())
}
