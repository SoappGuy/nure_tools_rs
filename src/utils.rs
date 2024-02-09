use crate::errors::{FindError, ParseError, RequestError};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Duration, Utc};
use chrono_tz::Tz::{self, Europe__Kiev};
use dateparser::parse;
use now::DateTimeNow;
use regex::Regex;
use reqwest::blocking::Response;
use serde_json::Value;
use std::fmt;

/** Period struct
**/
#[derive(Debug, Clone)]
pub struct Period {
    pub start_time: DateTime<Tz>,
    pub end_time: DateTime<Tz>,
}

impl Period {
    /** Create a new Period instance from a given String representations of a DateTime.

    Create a new Period instance from a given String representations of a DateTime.

    # Examples
    ```
    # use anyhow::Error;
    # use nure_tools::utils::Period;
    let start_time: &str = "2024-01-02";
    let end_time: &str = "January 3, 2024";

    let period: Period = Period::from_string(start_time, end_time)?;

    println!("Period: {:#?}", period);
    # Ok::<(), Error>(())
    ```
    # Errors
    This function fails if:
        [`ParseError::InvalidStringProvided`] - Can't parse datetime from given string.
    */
    pub fn from_string(start_time_str: &str, end_time_str: &str) -> Result<Self> {
        let start_time: DateTime<Tz> = match parse(start_time_str) {
            Ok(parsed) => parsed.with_timezone(&Europe__Kiev),
            Err(_) => {
                return Err(anyhow!(ParseError::InvalidStringProvided(String::from(
                    start_time_str
                ))));
            }
        };
        let end_time: DateTime<Tz> = match parse(end_time_str) {
            Ok(parsed) => parsed.with_timezone(&Europe__Kiev),
            Err(_) => {
                return Err(anyhow!(ParseError::InvalidStringProvided(String::from(
                    end_time_str
                ))));
            }
        };

        Ok(Self {
            start_time,
            end_time,
        })
    }

    /** Create a new Period instance from a given timestamp representations of a DateTime

    # Examples
    ```
    # use anyhow::Error;
    # use nure_tools::utils::Period;
    let start_time: i64 = 1704146400;
    let end_time: i64 = 1704232800;

    let period: Period = Period::from_timestamp(start_time, end_time)?;

    println!("Period: {:#?}", period);
    # Ok::<(), Error>(())
    ```
    # Errors
    This function fails if:
        [`ParseError::InvalidTimestampProvided`] - Can't parse datetime from given timestamp.
    */
    pub fn from_timestamp(start_time_i64: i64, end_time_i64: i64) -> Result<Self> {
        let start_time_i64 = start_time_i64.to_string();
        let end_time_i64 = end_time_i64.to_string();

        let start_time: DateTime<Tz> = match parse(&start_time_i64) {
            Ok(parsed) => parsed.with_timezone(&Europe__Kiev),
            Err(_) => {
                return Err(anyhow!(ParseError::InvalidTimestampProvided(
                    start_time_i64
                )));
            }
        };

        let end_time: DateTime<Tz> = match parse(&end_time_i64) {
            Ok(parsed) => parsed.with_timezone(&Europe__Kiev),
            Err(_) => {
                return Err(anyhow!(ParseError::InvalidTimestampProvided(end_time_i64)));
            }
        };

        Ok(Self {
            start_time,
            end_time,
        })
    }
    /** Create a new Period instance of day from current_time.

    # Examples
    ```
    # use anyhow::Error;
    # use nure_tools::utils::Period;

    let period: Period = Period::now();

    println!("Period: {:#?}", period);
    # Ok::<(), Error>(())
    ```
    # Errors
    This function fails if:
        [`ParseError::InvalidTimestampProvided`] - Can't parse datetime from given timestamp.

    */
    pub fn now() -> Self {
        let start_time: DateTime<Tz> = Utc::now().with_timezone(&Europe__Kiev);
        let end_time: DateTime<Tz> = start_time.end_of_day();

        Self {
            start_time,
            end_time,
        }
    }

    /** Create a new Period instance of current day borders

    # Examples
    ```
    # use anyhow::Error;
    # use nure_tools::utils::Period;
    let period: Period = Period::this_day();

    println!("Period: {:#?}", period);
    # Ok::<(), Error>(())
    ```
    **/
    pub fn this_day() -> Self {
        let today_date: DateTime<Tz> = Utc::now().with_timezone(&Europe__Kiev);

        let start_time: DateTime<Tz> = today_date.beginning_of_day();
        let end_time: DateTime<Tz> = today_date.end_of_day();

        Self {
            start_time,
            end_time,
        }
    }

    /** Create a new Period instance of next day borders

    # Examples
    ```
    # use anyhow::Error;
    # use nure_tools::utils::Period;
    let period: Period = Period::next_day();

    println!("Period: {:#?}", period);
    # Ok::<(), Error>(())
    ```
    **/
    pub fn next_day() -> Self {
        let today_date: DateTime<Tz> = Utc::now()
            .with_timezone(&Europe__Kiev)
            .checked_add_signed(Duration::days(1))
            .unwrap();

        let start_time: DateTime<Tz> = today_date.beginning_of_day();
        let end_time: DateTime<Tz> = today_date.end_of_day();

        Self {
            start_time,
            end_time,
        }
    }

    /** Create a new Period instance of 1 day from start_time
    # Examples:
    ```
    # use anyhow::Error;
    # use nure_tools::utils::Period;
    let period: Period = Period::day_from("2023-01-02")?;

    println!("Period: {:#?}", period);
    # Ok::<(), Error>(())
    ```
    # Errors
    This function fails if:
     [`ParseError::InvalidStringProvided`] - Can't parse datetime from given string.
     */
    pub fn day_from(start_time_str: &str) -> Result<Self> {
        let parsed_date: DateTime<Tz> = match parse(start_time_str) {
            Ok(parsed) => parsed.with_timezone(&Europe__Kiev),
            Err(_) => {
                return Err(anyhow!(ParseError::InvalidStringProvided(String::from(
                    start_time_str
                ))));
            }
        };

        let start_time: DateTime<Tz> = parsed_date.beginning_of_day();
        let end_time: DateTime<Tz> = parsed_date.end_of_day();

        Ok(Self {
            start_time,
            end_time,
        })
    }

    /** Create a new Period instance of current week borders

    # Examples
    ```
    # use anyhow::Error;
    # use nure_tools::utils::Period;
    let period: Period = Period::this_week();

    println!("Period: {:#?}", period);
    # Ok::<(), Error>(())
    ```
    **/
    pub fn this_week() -> Self {
        let today_date: DateTime<Tz> = Utc::now().with_timezone(&Europe__Kiev);

        let start_time: DateTime<Tz> = today_date.beginning_of_week();
        let end_time: DateTime<Tz> = today_date.end_of_week();
        Self {
            start_time,
            end_time,
        }
    }

    /** Create a new Period instance of next week borders

    # Examples
    ```
    # use anyhow::Error;
    # use nure_tools::utils::Period;
    let period: Period = Period::next_week();

    println!("Period: {:#?}", period);
    # Ok::<(), Error>(())
    ```
    **/
    pub fn next_week() -> Self {
        let today_date: DateTime<Tz> = Utc::now()
            .with_timezone(&Europe__Kiev)
            .checked_add_signed(Duration::weeks(1))
            .unwrap();

        let start_time: DateTime<Tz> = today_date.beginning_of_week();
        let end_time: DateTime<Tz> = today_date.end_of_week();

        Self {
            start_time,
            end_time,
        }
    }

    /** Create a new Period instance of 1 week from start_time
    # Examples:
    ```
    # use anyhow::Error;
    # use nure_tools::utils::Period;
    let period: Period = Period::week_from("2023-01-02")?;

    println!("Period: {:#?}", period);
    # Ok::<(), Error>(())
    ```
    # Errors
    This function fails if:
        [`ParseError::InvalidStringProvided`] - Can't parse datetime from given string.
     */
    pub fn week_from(start_time_str: &str) -> Result<Self> {
        let parsed_date: DateTime<Tz> = match parse(start_time_str) {
            Ok(parsed) => parsed.with_timezone(&Europe__Kiev),
            Err(_) => {
                return Err(anyhow!(ParseError::InvalidStringProvided(String::from(
                    start_time_str
                ))));
            }
        };

        let start_time: DateTime<Tz> = parsed_date.beginning_of_week();
        let end_time = parsed_date.end_of_week();

        Ok(Self {
            start_time,
            end_time,
        })
    }
}

impl fmt::Display for Period {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "start_time: {}, end_time: {}",
            self.start_time, self.end_time
        )
    }
}

/** Function to say if `find_it` presented in `search_here` using RegExp.
# Arguments
 * `find_it` - the string to be found.
 * `search_here` - the string in which to look at `find_it`.

# Examples
```
# use anyhow::Error;
# use nure_tools::utils::find;
let find_it: &str = "пі";
let search_here: &str = "пзпі-23-2";

println!(
    "{}",
    if find(find_it, search_here)? {
        "found!"
    } else {
        "nothing :("
    }
);
# Ok::<(), Error>(())
```

# Errors
This function fails if:
 * [`FindError::InvalidRegexString`] - Regex engine can't parse given string for any reason.
**/
pub fn find(find_it: &str, search_here: &str) -> Result<bool> {
    let regex: Regex = match Regex::new(find_it.to_lowercase().as_str()) {
        Ok(compiled) => compiled,
        Err(_) => {
            return Err(anyhow!(FindError::InvalidRegexString(String::from(
                find_it
            ))));
        }
    };

    Ok(regex.find(search_here.to_lowercase().as_str()).is_some())
}

/** Helper function to catch errors while waiting for Get result.

You probably will never use it, but you can if you want, see example in [get_groups]/[get_teachers]/[get_lecture_rooms]/[get_schedule] functions sources.

[get_groups]: `crate::groups::get_groups`
[get_teachers]: `crate::teachers::get_teachers`
[get_lecture_rooms]: `crate::lecture_rooms::get_lecture_rooms`
[get_schedule]: `crate::schedule::get_schedule`
**/
pub fn get_wrapper(get_response: reqwest::Result<Response>) -> Result<Value> {
    match get_response {
        Ok(value) => match value.status().as_u16() {
            200 => match value.json::<serde_json::Value>() {
                Ok(value) => Ok(value),
                Err(_) => Err(anyhow!(RequestError::NotJson)),
            },
            _ => Err(anyhow!(RequestError::BadResponse(
                String::from(value.status().canonical_reason().unwrap_or("")),
                value.status().as_u16()
            ))),
        },

        Err(_) => Err(anyhow!(RequestError::GetFailed)),
    }
}
