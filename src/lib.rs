mod components;

use std::fs::read_to_string;

use anathema::prelude::*;
use eyre::{Context, Result};

pub fn run() -> Result<()> {
    let index_template = read_to_string("templates/index.aml").context("loading index template")?;
    let doc = Document::new(index_template);
    let backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_mouse()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .context("creating backend")?;
    let mut runtime_builder = Runtime::builder(doc, backend);

    components::register_components(&mut runtime_builder).context("registering components")?;

    let mut runtime = runtime_builder.finish().context("creating runtime")?;

    runtime.fps = 128;
    runtime.run().context("running project")
}
