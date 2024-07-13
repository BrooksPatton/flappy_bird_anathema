use std::fs::read_to_string;

use anathema::{
    backend::tui::TuiBackend,
    component::Component,
    runtime::RuntimeBuilder,
    state::{State, Value},
};
use eyre::{Context, Result};

pub struct Bird;

impl Component for Bird {
    type State = BirdState;

    type Message = ();

    fn tick(
        &mut self,
        state: &mut Self::State,
        _elements: anathema::widgets::Elements<'_, '_>,
        _viewport: anathema::prelude::Viewport,
        dt: std::time::Duration,
    ) {
        let delta = dt.as_secs_f32();

        state.apply_gravity();
        state.apply_movement(delta);
        state.update_tick();
    }

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        state: &mut Self::State,
        _elements: anathema::widgets::Elements<'_, '_>,
        _viewport: anathema::prelude::Viewport,
    ) {
        if key.get_char().unwrap_or('q') == ' ' {
            state.jump();
        }
    }
}

pub fn register(runtime: &mut RuntimeBuilder<TuiBackend>) -> Result<()> {
    runtime.register_component(
        "bird",
        read_to_string("templates/bird.aml").context("reading template")?,
        Bird,
        BirdState::default(),
    );

    Ok(())
}

#[derive(State, Debug)]
pub struct BirdState {
    pub height: Value<f32>,
    pub ticks: Value<usize>,
    pub velocity: Value<f32>,
    pub acceleration: Value<f32>,
    pub gravity: Value<f32>,
    pub jump_force: Value<f32>,
}

impl BirdState {
    fn update_tick(&mut self) {
        let mut ticks = self.ticks.copy_value();

        ticks += 1;

        self.ticks.set(ticks);
    }

    pub fn apply_gravity(&mut self) {
        let mut acceleration = self.acceleration.copy_value();
        let gravity = self.gravity.copy_value();

        acceleration += gravity;

        self.acceleration.set(acceleration);
    }

    pub fn apply_movement(&mut self, delta: f32) {
        let mut acceleration = self.acceleration.copy_value();
        let mut velocity = self.velocity.copy_value();
        let mut height = self.height.copy_value();

        velocity += acceleration * delta;
        height += velocity;
        acceleration *= 0.;

        self.acceleration.set(acceleration);
        self.velocity.set(velocity);
        self.height.set(height);
    }

    pub fn jump(&mut self) {
        let mut velocity = self.velocity.copy_value();
        let jump_force = self.jump_force.copy_value();
        let acceleration = jump_force;

        velocity *= 0.;

        self.acceleration.set(acceleration);
        self.velocity.set(velocity);
    }
}

impl Default for BirdState {
    fn default() -> Self {
        Self {
            velocity: Default::default(),
            gravity: Value::new(1.),
            height: Default::default(),
            ticks: Default::default(),
            acceleration: Default::default(),
            jump_force: Value::new(-50.),
        }
    }
}
