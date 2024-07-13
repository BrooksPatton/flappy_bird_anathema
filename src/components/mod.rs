use anathema::{backend::tui::TuiBackend, runtime::RuntimeBuilder};
use eyre::{Context, Result};

pub mod bird;
pub mod pipe;

pub fn register_components(runtime: &mut RuntimeBuilder<TuiBackend>) -> Result<()> {
    bird::register(runtime).context("registering bird component")?;
    pipe::register(runtime).context("registering pipe component")?;

    Ok(())
}
