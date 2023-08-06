use eyre::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    xtaskops::tasks::main().ok();
    update_badge_for_coverage()?;
    Ok(())
}

/// Update bdage file when the badge is not up to date
///
/// Get the modified time of the badge file (flat.svg) and compare it to the 
/// flat.svg in assets directory. If the badge is not up to date, copy the
/// badge from coverage/html/badges/flat.svg to assets/flat.svg
fn update_badge_for_coverage() -> Result<()> {
    let coverage_badge = PathBuf::from("coverage/html/badges/flat.svg");
    let assets_badge = PathBuf::from("assets/flat.svg");
    if coverage_badge.exists() && assets_badge.exists() {
        let coverage_badge_modified = coverage_badge.metadata()?.modified()?;
        let assets_badge_modified = assets_badge.metadata()?.modified()?;
        if coverage_badge_modified > assets_badge_modified {
            std::fs::copy(coverage_badge, assets_badge)?;
        }
    } else if coverage_badge.exists() && !assets_badge.exists() {
        std::fs::copy(coverage_badge, assets_badge)?;
    }
    Ok(())
}
