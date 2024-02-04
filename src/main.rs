use anyhow::Result;
use nure_tools::utils::find;

fn main() -> Result<()> {
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

    Ok(())
}
