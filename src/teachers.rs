use crate::{
    errors::{FindError, RequestError},
    utils::{find, get_wrapper},
};
use anyhow::{anyhow, Result};
use reqwest::blocking::get;
use serde_json::{self, Value};

/** Helper function to parse teacher json returned by API into [`Teacher`] struct.

You probably will never use it, but you can if you want, see example in [`get_teachers`] function source
**/
pub fn parse_teacher_json(vector: Vec<Value>) -> Vec<Teacher> {
    let mut result: Vec<Teacher> = Vec::new();

    let mut id: i32 = 0;
    let mut short_name: String = String::new();
    let mut long_name: String = String::new();

    for element in vector {
        if let Value::Object(obj) = element {
            if let Some(Value::Number(n)) = obj.get("id") {
                id = n.as_i64().unwrap_or(0) as i32;
            }
            if let Some(Value::String(st)) = obj.get("shortName") {
                short_name = st.clone();
            }
            if let Some(Value::String(st)) = obj.get("fullName") {
                long_name = st.clone();
            }
            result.push(Teacher::new(id, short_name.clone(), long_name.clone()));
        };
    }

    result
}

/** Get all existing teachers.

Returns all existing teachers in `Vec<Teacher>` format.

# Examples
```
# use nure_tools::teachers::{get_teachers, Teacher};
# use anyhow::Error;
let teachers: Vec<Teacher> = get_teachers()?;
println!("{:#?}", teachers);
# Ok::<(), Error>(())
```

# Errors
This function fails if:
 * `RequestError::GetFailed` - Get request fails.
 * `RequestError::NotJson` - Server returns value not in json format.
 * `RequestError::BadResponse` - Server returns any response except 200.
 * `RequestError::InvalidReturn` - Server returns value in unexpected format.
**/
pub fn get_teachers() -> Result<Vec<Teacher>> {
    let response = get_wrapper(get("https://api.mindenit.tech/teachers"))?;
    if let Value::Array(vector) = response {
        let result: Vec<Teacher> = parse_teacher_json(vector);
        Ok(result)
    } else {
        Err(anyhow!(RequestError::InvalidReturn))
    }
}

/** Find a Teacher by name.

Returns all matched teachers in `Vec<Teacher>` format.

# Arguments

* `name` - &str with teacher name to search for.

# Examples
```
# use anyhow::Error;
# use nure_tools::teachers::{find_teacher, Teacher};
let teacher: Vec<Teacher> = find_teacher("Новіков")?;
println!("teachers: {:#?}\n", teacher);

let teacher: Vec<Teacher> = find_teacher("Гліб")?;
println!("teachers: {:#?}\n", teacher);
# Ok::<(), Error>(())
```

# Errors
This function fails if:
 * `FindError::InvalidTeacherName(name)` - There is no teacher that matches given name.
 * [`get_teachers`] fails.
 * [`find`] fails.
**/
pub fn find_teacher(name: &str) -> Result<Vec<Teacher>> {
    let teachers = get_teachers()?;
    let mut result: Vec<Teacher> = vec![];

    for teacher in teachers {
        if find(name, &teacher.full_name)? {
            result.push(teacher);
        } else {
            continue;
        }
    }

    if result.is_empty() {
        Err(anyhow!(FindError::InvalidTeacherName(String::from(name))))
    } else {
        Ok(result)
    }
}

/** Find exect teacher.

Returns 1 exect matched teacher.

# Arguments

* `name` - &str with teacher name to search for.

# Examples
```
# use anyhow::Error;
# use nure_tools::teachers::{find_exect_teacher, Teacher};
let teacher: Teacher = find_exect_teacher("Терещенко Г. Ю.")?;
println!("teacher: {:#?}", teacher);
# Ok::<(), Error>(())
```

# Errors
This function fails if:
 * `FindError::InvalidTeacherName(name)` - There is no teacher that matches given name.
 * [`get_teachers`] fails.
**/
pub fn find_exect_teacher(name: &str) -> Result<Teacher> {
    let teacher = get_teachers()?;

    for teacher in teacher {
        if name.to_lowercase() == teacher.short_name.to_lowercase() {
            return Ok(teacher);
        } else {
            continue;
        }
    }

    Err(anyhow!(FindError::InvalidTeacherName(String::from(name))))
}

/** Teacher struct.
**/
#[derive(Debug, Clone)]
pub struct Teacher {
    pub id: i32,
    pub short_name: String,
    pub full_name: String,
}

impl Teacher {
    fn new(id: i32, short_name: String, full_name: String) -> Self {
        Self {
            id,
            short_name,
            full_name,
        }
    }
}

impl std::fmt::Display for Teacher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_name)
    }
}
