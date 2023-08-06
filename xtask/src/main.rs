use eyre::Result;

fn main() -> Result<()> {
    xtaskops::tasks::main().ok();
    Ok(())
}
