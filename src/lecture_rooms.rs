use crate::utils::find;
use color_eyre::{eyre::eyre, Result};
use reqwest::blocking::get;
use serde_json::{self, Value};

/** Helper function to parse lecture_room json returned by API into [`LectureRoom`] struct.

You probably will never use it, but you can if you want, see example in [`get_lecture_rooms`] function source
**/
pub fn parse_lecture_room_json(vector: Vec<Value>) -> Vec<LectureRoom> {
    let mut result: Vec<LectureRoom> = Vec::new();

    let mut id: i32 = 0;
    let mut name: String = String::new();

    for element in vector {
        if let Value::Object(obj) = element {
            if let Value::Number(n) = obj.get("id").unwrap() {
                id = n.as_i64().unwrap() as i32;
            }
            if let Value::String(st) = obj.get("name").unwrap() {
                name = st.clone();
            }
            result.push(LectureRoom::new(id, name.clone()));
        };
    }

    result
}

/** Get all existing lecture_rooms

Returns all existing lecture rooms in `Vec<LectureRoom>` format.

# Examples
```
use color_eyre::Result;
use nure_tools::lecture_rooms::{get_lecture_rooms, LectureRoom};

fn main() -> Result<()> {
    color_eyre::install()?;

    let lecture_rooms: Vec<LectureRoom> = get_lecture_rooms()?;
    println!("{:#?}", lecture_rooms);

    Ok(())
}
```
**/
pub fn get_lecture_rooms() -> Result<Vec<LectureRoom>> {
    let response = get("https://api.mindenit.tech/auditories")?.json::<serde_json::Value>()?;

    if let Value::Array(vector) = response {
        let result: Vec<LectureRoom> = parse_lecture_room_json(vector);
        Ok(result)
    } else {
        Err(eyre!("Can't get lecture_rooms, bad response {}", response))
    }
}

/** Find a lecture_room by it name

Returns all matched lecture_rooms in `Vec<LectureRoom>` format.

# Arguments

* `name` - &str with lecture_room name to search for.

# Examples
```
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
```
**/

pub fn find_lecture_room(name: &str) -> Result<Vec<LectureRoom>> {
    let lecture_rooms = get_lecture_rooms()?;
    let mut result: Vec<LectureRoom> = vec![];

    for lecture_room in lecture_rooms {
        if find(name, &lecture_room.name) {
            result.push(lecture_room);
        } else {
            continue;
        }
    }

    Ok(result)
}

/** LectureRoom struct.
**/
#[derive(Debug)]
pub struct LectureRoom {
    pub id: i32,
    pub name: String,
}

impl LectureRoom {
    fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }
}
