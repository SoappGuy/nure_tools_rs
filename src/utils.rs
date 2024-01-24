use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use chrono_tz::{
    Europe::Kiev,
    Tz::{self, Europe__Kiev},
};
use dateparser::parse;
use regex::Regex;

/** Period struct
**/
#[derive(Debug, Clone)]
pub struct Period {
    start_time: DateTime<Tz>,
    end_time: DateTime<Tz>,
}

impl Period {
    /** Create a new Period instance from a given String representations of a DateTime
    # Arguments
       * `start_time` - &str with start time in supported format.
       * `end_time` - &str with end time in supported format.

    # Examples
    ```
    use color_eyre::Result;
    use nure_tools::utils::Period;

    fn main() -> Result<()> {
        color_eyre::install()?;

        let start_time: &str = "2024-01-02";
        let end_time: &str = "January 3, 2024";

        let period: Period = Period::from_string(start_time, end_time);

        println!("Period: {:#?}", period);

        Ok(())
    }

    ```
    **/
    pub fn from_string(start_time: &str, end_time: &str) -> Self {
        let start_time: DateTime<Tz> = parse(start_time).unwrap().with_timezone(&Europe__Kiev);
        let end_time: DateTime<Tz> = parse(end_time).unwrap().with_timezone(&Europe__Kiev);

        Self {
            start_time,
            end_time,
        }
    }

    /** Create a new Period instance from a given timestamp representations of a DateTime
    # Arguments
       * `start_time` - i64 with start time in unix timestamp format.
       * `end_time` - i64 with end time in unix timestamp format.

    # Examples
    ```
    use color_eyre::Result;
    use nure_tools::utils::Period;

    fn main() -> Result<()> {
        color_eyre::install()?;

        let start_time: i64 = 1704146400;
        let end_time: i64 = 1704232800;

        let period: Period = Period::from_timestamp(start_time, end_time);

        println!("Period: {:#?}", period);

        Ok(())
    }
    ```
    **/
    pub fn from_timestamp(start_time: i64, end_time: i64) -> Self {
        let start_time = parse(&start_time.to_string())
            .unwrap()
            .with_timezone(&Europe__Kiev);
        let end_time = parse(&end_time.to_string())
            .unwrap()
            .with_timezone(&Europe__Kiev);

        Self {
            start_time,
            end_time,
        }
    }
}

/** Function to say if `find_it` presented in `search_here` using RegExp.
# Arguments
 * `find_it` - the string to be found.
 * `search_here` - the string in which to look at `find_it`.

# Examples
```
use color_eyre::Result;
use nure_tools::utils::find;

fn main() -> Result<()> {
    color_eyre::install()?;

    let find_it: &str = "пі";
    let search_here: &str = "пзпі-23-2";

    println!(
        "{}",
        if find(find_it, search_here) {
            "found!"
        } else {
            "nothing :("
        }
    );

    Ok(())
}
```
**/
pub fn find(find_it: &str, search_here: &str) -> bool {
    let regex = Regex::new(find_it.to_lowercase().as_str()).unwrap();
    regex.find(search_here.to_lowercase().as_str()).is_some()
}
