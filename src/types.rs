use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub boost: f32,
    pub gravity: f32,
}

pub struct Boost(pub f32);

pub struct Gravity(pub f32);

pub struct Ball;

pub struct Position(pub f32);

pub struct Velocity(pub f32);
