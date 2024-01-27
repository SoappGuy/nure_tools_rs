use crate::utils::find;
use color_eyre::{eyre::eyre, Result};
use reqwest::blocking::get;
use serde_json::{self, Value};

/** Helper function to parse group json returned by API into [`Group`] struct.

You probably will never use it, but you can if you want, see example in [`get_groups`] function source
**/
pub fn parse_group_json(vector: Vec<Value>) -> Vec<Group> {
    let mut result: Vec<Group> = Vec::new();

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
            result.push(Group::new(id, name.clone()));
        };
    }

    result
}

/** Get all existing groups.

Returns all existing groups in `Vec<Group>` format.

# Examples
```
use color_eyre::Result;
use nure_tools::groups::{get_groups, Group};

fn main() -> Result<()> {
    color_eyre::install()?;

    let groups: Vec<Group> = get_groups()?;
    println!("{:#?}", groups);

    Ok(())
}
```
**/
pub fn get_groups() -> Result<Vec<Group>> {
    let response = get("https://api.mindenit.tech/groups")?.json::<serde_json::Value>()?;

    if let Value::Array(vector) = response {
        let result: Vec<Group> = parse_group_json(vector);
        Ok(result)
    } else {
        Err(eyre!("Can't get groups, bad response {}", response))
    }
}

/** Find a group by it name.

Returns all matched groups in `Vec<Group>` format.

# Arguments

* `name` - &str with group name to search for.

# Examples
```
use color_eyre::Result;
use nure_tools::groups::{find_group, Group};

fn main() -> Result<()> {
    color_eyre::install()?;

    let group: Vec<Group> = find_group("пзпі-23-2")?;
    println!("groups: {:#?}\n", group);

    let group: Vec<Group> = find_group("пі-23")?;
    println!("groups: {:#?}\n", group);

    Ok(())
}
```
**/
pub fn find_group(name: &str) -> Result<Vec<Group>> {
    let groups = get_groups()?;
    let mut result: Vec<Group> = vec![];

    for group in groups {
        if find(name, &group.name) {
            result.push(group);
        } else {
            continue;
        }
    }

    Ok(result)
}

/** Find exect group.

Returns 1 exect matched group.

# Arguments

* `name` - &str with group name to search for.

# Examples
```
use color_eyre::Result;
use nure_tools::groups::{find_exect_group, Group};

fn main() -> Result<()> {
    color_eyre::install()?;

    let group: Group = find_exect_group("пзпі-23-2")?;
    println!("group: {:#?}", group);

    Ok(())
}
```
**/
pub fn find_exect_group(name: &str) -> Result<Group> {
    let groups = get_groups()?;

    for group in groups {
        if name.to_lowercase() == group.name.to_lowercase() {
            return Ok(group);
        } else {
            continue;
        }
    }

    Err(eyre!("There is no group with exect name {}", name))
}

/** Group struct.
**/
#[derive(Debug, Clone)]
pub struct Group {
    pub id: i32,
    pub name: String,
}

impl Group {
    fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }
}
