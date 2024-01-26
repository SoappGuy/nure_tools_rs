use chrono::{DateTime, Duration, Utc};
use chrono_tz::Tz::{self, Europe__Kiev};
use dateparser::parse;
use now::DateTimeNow;
use regex::Regex;

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
        let start_time: DateTime<Tz> = parse(&start_time.to_string())
            .unwrap()
            .with_timezone(&Europe__Kiev);
        let end_time: DateTime<Tz> = parse(&end_time.to_string())
            .unwrap()
            .with_timezone(&Europe__Kiev);

        Self {
            start_time,
            end_time,
        }
    }

    /** Create a new Period instance of current day borders

    # Examples
    ```
    use color_eyre::Result;
    use nure_tools::utils::Period;

    fn main() -> Result<()> {
        color_eyre::install()?;

        let period: Period = Period::today();

        println!("Period: {:#?}", period);

        Ok(())
    }
    ```
    **/
    pub fn today() -> Self {
        let today_date: DateTime<Tz> = Utc::now().with_timezone(&Europe__Kiev);

        let start_time: DateTime<Tz> = today_date.beginning_of_day();
        let end_time: DateTime<Tz> = today_date.end_of_day();

        Self {
            start_time,
            end_time,
        }
    }

    /** Create a new Period instance of current week borders

    # Examples
    ```
    use color_eyre::Result;
    use nure_tools::utils::Period;

    fn main() -> Result<()> {
        color_eyre::install()?;

        let period: Period = Period::this_week();

        println!("Period: {:#?}", period);

        Ok(())
    }
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

    pub fn week_from(start_time: &str) -> Self {
        let parsed_date = parse(start_time).unwrap().with_timezone(&Europe__Kiev);

        let start_time: DateTime<Tz> = parsed_date.beginning_of_week();

        let end_time = parsed_date.end_of_week();

        Self {
            start_time,
            end_time,
        }
    }

    /** Create a new Period instance of next week borders

    # Examples
    ```
    use color_eyre::Result;
    use nure_tools::utils::Period;

    fn main() -> Result<()> {
        color_eyre::install()?;

        let period: Period = Period::next_week();

        println!("Period: {:#?}", period);

        Ok(())
    }
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
