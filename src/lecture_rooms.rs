use crate::{
    errors::{FindError, RequestError},
    utils::{find, get_wrapper},
};
use anyhow::{anyhow, Result};
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
            if let Some(Value::Number(n)) = obj.get("id") {
                id = n.as_i64().unwrap_or(0) as i32;
            }

            if let Some(Value::String(st)) = obj.get("name") {
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
# use nure_tools::lecture_rooms::{get_lecture_rooms, LectureRoom};
# use anyhow::Error;
let lecture_rooms: Vec<LectureRoom> = get_lecture_rooms()?;
println!("{:#?}", lecture_rooms);
# Ok::<(), Error>(())
```

# Errors
This function fails if:
 * `RequestError::GetFailed` - Get request fails.
 * `RequestError::NotJson` - Server returns value not in json format.
 * `RequestError::BadResponse` - Server returns any response except 200.
 * `RequestError::InvalidReturn` - Server returns value in unexpected format.
**/
pub fn get_lecture_rooms() -> Result<Vec<LectureRoom>> {
    let response = get_wrapper(get("https://api.mindenit.tech/auditories"))?;
    if let Value::Array(vector) = response {
        let result: Vec<LectureRoom> = parse_lecture_room_json(vector);
        Ok(result)
    } else {
        Err(anyhow!(RequestError::InvalidReturn))
    }
}

/** Find a lecture_room by it name

Returns all matched lecture_rooms in `Vec<LectureRoom>` format.

# Arguments

* `name` - &str with lecture_room name to search for.

# Examples
```
# use anyhow::Error;
# use nure_tools::lecture_rooms::{find_lecture_room, LectureRoom};
let lecture_room: Vec<LectureRoom> = find_lecture_room("і")?;
println!("lecture_rooms: {:#?}\n", lecture_room);

let lecture_room: Vec<LectureRoom> = find_lecture_room("філія")?;
println!("lecture_rooms: {:#?}\n", lecture_room);
# Ok::<(), Error>(())
```

# Errors
This function fails if:
 * `FindError::InvalidLectureRoomName(name)` - There is no lecture_room that matches given name.
 * [`get_lecture_rooms`] fails.
 * [`find`] fails.
**/
pub fn find_lecture_room(name: &str) -> Result<Vec<LectureRoom>> {
    let lecture_rooms = get_lecture_rooms()?;
    let mut result: Vec<LectureRoom> = vec![];

    for lecture_room in lecture_rooms {
        if find(name, &lecture_room.name)? {
            result.push(lecture_room);
        } else {
            continue;
        }
    }

    if result.is_empty() {
        Err(anyhow!(FindError::InvalidLectureRoomName(String::from(
            name
        ))))
    } else {
        Ok(result)
    }
}

/** Find exect lecture_room.

Returns 1 exect matched lecture_room.

# Arguments

* `name` - &str with lecture_room name to search for.

# Examples
```
# use anyhow::Error;
# use nure_tools::lecture_rooms::{find_exect_lecture_room, LectureRoom};
let lecture_room: LectureRoom = find_exect_lecture_room("ФІЛІЯ")?;
println!("lecture_room: {:#?}", lecture_room);
# Ok::<(), Error>(())
```

# Errors
This function fails if:
 * `FindError::InvalidLectureRoomName(name)` - There is no lecture_room that matches given name.
 * [`get_lecture_rooms`] fails.
**/
pub fn find_exect_lecture_room(name: &str) -> Result<LectureRoom> {
    let lecture_rooms = get_lecture_rooms()?;

    for lecture_rooms in lecture_rooms {
        if name.to_lowercase() == lecture_rooms.name.to_lowercase() {
            return Ok(lecture_rooms);
        } else {
            continue;
        }
    }

    Err(anyhow!(FindError::InvalidGroupName(String::from(name))))
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

impl std::fmt::Display for LectureRoom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
