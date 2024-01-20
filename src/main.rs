use color_eyre::Result;
use nure_tools::utils::find;

fn main() -> Result<()> {
    color_eyre::install()?;

    let find_it = "пі";
    let search_here = "пзпі-23-2";

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
