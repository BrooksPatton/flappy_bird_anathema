use std::fs::read_to_string;

use anathema::{
    component::Component,
    prelude::*,
    runtime::RuntimeBuilder,
    state::{State, Value},
};
use eyre::{Context, Result};

pub struct Pipe;

impl Component for Pipe {
    type State = PipeState;

    type Message = ();
}

#[derive(State, Debug)]
pub struct PipeState {
    pub x: Value<f32>,
    pub y: Value<f32>,
    pub width: Value<f32>,
    pub height: Value<f32>,
}

impl PipeState {
    pub fn new() -> Self {
        let x = Value::new(100.);
        let y = Value::new(50.);
        let width = Value::new(5.);
        let height = Value::new(10.);

        Self {
            x,
            y,
            width,
            height,
        }
    }
}

pub fn register(runtime: &mut RuntimeBuilder<TuiBackend>) -> Result<()> {
    runtime.register_component(
        "pipe",
        read_to_string("templates/pipe.aml").context("reading template")?,
        Pipe,
        PipeState::new(),
    );

    Ok(())
}
