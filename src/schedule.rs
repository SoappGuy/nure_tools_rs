use crate::{
    groups::{parse_group_json, Group},
    lecture_rooms::LectureRoom,
    teachers::{parse_teacher_json, Teacher},
    utils::Period,
};
use color_eyre::Result;
use reqwest::blocking::get;
use serde_json::{self, Map, Value};

/** Get schedule function.

Returns shedule for the given request in `Vec<Lecture>` format.

# Arguments
 * request - accepts a [`Request`] enum with a [`Group`]/[`Teacher`]/[`LectureRoom`] object inside.
 * period - accepts a [`Period`] struct.

# Examples
```
use color_eyre::Result;
use nure_tools::{
    groups::{find_group, Group},
    schedule::{get_schedule, Lecture, Request},
};
fn main() -> Result<()> {
    color_eyre::install()?;

    let groups_response: Vec<Group> = find_group(String::from("пзпі-23-2"))?;

    for group in groups_response {
        let schedule_request_bygroup: Request = Request::Group(group);
        let schedule_response: Vec<Lecture> = get_schedule(
            schedule_request_bygroup,
            String::from("2024-01-02"),
            String::from("2024-01-03"),
        )?;
        print!("{:#?}", schedule_response);
    }

    Ok(())
}
```
**/

pub fn get_schedule(request: Request, period: Period) -> Result<Vec<Lecture>> {
    // start_time = convert_time_to_timestamp(start_time);
    // end_time = convert_time_to_timestamp(end_time);

    let start_time = period.start_time.timestamp().to_string();
    let end_time = period.end_time.timestamp().to_string();

    let (request_type, request_id) = match request {
        Request::Group(group) => ("group", group.id),
        Request::Teacher(teacher) => ("teacher", teacher.id),
        Request::LectureRoom(lecture_room) => ("auditory", lecture_room.id),
    };

    let response = get(format!(
        "https://api.mindenit.tech/schedule?type={}&id={}&start_time={}&end_time={}",
        request_type, request_id, start_time, end_time,
    ))?
    .json::<serde_json::Value>()?;

    let mut result: Vec<Lecture> = Vec::new();

    if let Value::Array(vector) = response {
        let mut lecture_room: String = String::new();
        let mut start_time: i64 = 0;
        let mut end_time: i64 = 0;
        let mut number_pair: u8 = 0;
        let mut lecture_type: String = String::new();
        let mut teachers: Vec<Teacher> = vec![];
        let mut groups: Vec<Group> = vec![];
        let mut subject: Subject = Subject::default();

        for element in vector {
            if let Value::Object(mut obj) = element {
                if let Value::String(st) = obj.get("auditory").unwrap() {
                    lecture_room = st.clone();
                }
                if let Value::Number(n) = obj.get("startTime").unwrap() {
                    start_time = n.as_i64().unwrap();
                }
                if let Value::Number(n) = obj.get("endTime").unwrap() {
                    end_time = n.as_i64().unwrap();
                }
                if let Value::Number(n) = obj.get("numberPair").unwrap() {
                    number_pair = n.as_i64().unwrap() as u8;
                }
                if let Value::String(st) = obj.get("type").unwrap() {
                    lecture_type = st.clone();
                }
                if let Value::Array(vector) = obj.remove("teachers").unwrap() {
                    teachers = parse_teacher_json(vector);
                }
                if let Value::Array(vector) = obj.remove("groups").unwrap() {
                    groups = parse_group_json(vector);
                }
                if let Value::Object(obj) = obj.remove("subject").unwrap() {
                    subject = parse_subject_json(obj);
                }

                result.push(Lecture::new(
                    lecture_room.clone(),
                    Period::from_timestamp(start_time, end_time),
                    number_pair,
                    lecture_type.clone(),
                    teachers.clone(),
                    groups.clone(),
                    subject.clone(),
                ));
            };
        }
    }

    Ok(result)
}

/** Helper function to parse subject json returned by API into [`Subject`] struct.

You probably will never use it, but you can if you want, see example in [`get_schedule`] function source
**/
pub fn parse_subject_json(obj: Map<String, Value>) -> Subject {
    let mut brief: String = String::new();
    let mut id: i32 = 0;
    let mut title: String = String::new();

    if let Value::String(st) = obj.get("brief").unwrap() {
        brief = st.clone();
    }
    if let Value::Number(n) = obj.get("id").unwrap() {
        id = n.as_i64().unwrap() as i32;
    }
    if let Value::String(st) = obj.get("title").unwrap() {
        title = st.clone();
    }

    Subject::new(brief, id, title)
}

/** Request enum to simplify the [`get_schedule`] function.
# Variants
 * `Group` - require a [`Group`] to parse id from it.
 * `Teacher` - require a [`Teacher`] to parse id from it.
 * `LectureRoom` - require a [`LectureRoom`] to parse id from it.
**/
pub enum Request {
    Group(Group),
    Teacher(Teacher),
    LectureRoom(LectureRoom),
}

/** Massive Lacture struct.
**/
#[derive(Debug, Clone)]
pub struct Lecture {
    pub lecture_room: String,
    pub period: Period,
    pub number_pair: u8,
    pub lecture_type: String,
    pub teachers: Vec<Teacher>,
    pub groups: Vec<Group>,
    pub subject: Subject,
}
/** Subject struct.
**/
#[derive(Debug, Clone)]
pub struct Subject {
    pub brief: String,
    pub id: i32,
    pub title: String,
}

impl Lecture {
    fn new(
        lecture_room: String,
        period: Period,
        number_pair: u8,
        lecture_type: String,
        teachers: Vec<Teacher>,
        groups: Vec<Group>,
        subject: Subject,
    ) -> Self {
        Self {
            lecture_room,
            period,
            number_pair,
            lecture_type,
            teachers,
            groups,
            subject,
        }
    }
}

impl Subject {
    fn new(brief: String, id: i32, title: String) -> Self {
        Self { brief, id, title }
    }
}

impl Default for Subject {
    fn default() -> Self {
        Self {
            brief: String::from("empty"),
            id: 0,
            title: String::from("empty"),
        }
    }
}
