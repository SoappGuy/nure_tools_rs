use crate::{
    errors::{FindError, RequestError},
    utils::{find, get_wrapper},
};
use anyhow::{anyhow, Result};
use reqwest::blocking::get;
use serde_json::Value;

/** Helper function to parse group json returned by API into [`Group`] struct.

You probably will never use it, but you can if you want, see example in [`get_groups`] function source
**/
pub fn parse_group_json(vector: Vec<Value>) -> Vec<Group> {
    let mut result: Vec<Group> = Vec::new();

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
            result.push(Group::new(id, name.clone()));
        };
    }

    result
}

/** Get all existing groups.

Returns all existing groups in `Vec<Group>` format.

# Examples
```
# use nure_tools::groups::{get_groups, Group};
# use anyhow::Error;
let groups: Vec<Group> = get_groups()?;
println!("{:#?}", groups);
# Ok::<(), Error>(())
```

# Errors
This function fails if:
 * `RequestError::GetFailed` - Get request fails.
 * `RequestError::NotJson` - Server returns value not in json format.
 * `RequestError::BadResponse` - Server returns any response except 200.
 * `RequestError::InvalidReturn` - Server returns value in unexpected format.
**/
pub fn get_groups() -> Result<Vec<Group>> {
    let response = get_wrapper(get("https://api.mindenit.tech/lists/groups"))?;
    if let Value::Array(vector) = response {
        let result: Vec<Group> = parse_group_json(vector);
        Ok(result)
    } else {
        Err(anyhow!(RequestError::InvalidReturn))
    }
}

/** Find a group by it name.

Returns all matched groups in `Vec<Group>` format.

# Arguments

* `name` - &str with group name to search for.

# Examples
```
# use anyhow::Error;
# use nure_tools::groups::{find_group, Group};
let group: Vec<Group> = find_group("пзпі-23-2")?;
println!("groups: {:#?}\n", group);

let group: Vec<Group> = find_group("пі-23")?;
println!("groups: {:#?}\n", group);
# Ok::<(), Error>(())
```

# Errors
This function fails if:
 * `FindError::InvalidGroupName(name)` - There is no group that matches given name.
 * [`get_groups`] fails.
 * [`find`] fails.
**/
pub fn find_group(name: &str) -> Result<Vec<Group>> {
    let groups = get_groups()?;
    let mut result: Vec<Group> = vec![];

    for group in groups {
        if find(name, &group.name)? {
            result.push(group);
        } else {
            continue;
        }
    }

    if result.is_empty() {
        Err(anyhow!(FindError::InvalidGroupName(String::from(name))))
    } else {
        Ok(result)
    }
}

/** Find exect group.

Returns 1 exect matched group.

# Arguments

* `name` - &str with group name to search for.

# Examples
```
# use anyhow::Error;
# use nure_tools::groups::{find_exect_group, Group};
let group: Group = find_exect_group("пзпі-23-2")?;
println!("group: {:#?}", group);
# Ok::<(), Error>(())
```

# Errors
This function fails if:
 * `FindError::InvalidGroupName(name)` - There is no group that matches given name.
 * [`get_groups`] fails.
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

    Err(anyhow!(FindError::InvalidGroupName(String::from(name))))
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

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
