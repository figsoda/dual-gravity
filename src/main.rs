mod system;
mod types;

use crate::{
    system::{boost, game_over, gravity, movement, render, startup},
    types::{Boost, Config, Gravity},
};

use bevy::{prelude::*, render::pass::ClearColor};

use std::fs;

fn main() {
    let cfg: Config = toml::from_slice(&fs::read("dgcfg.toml").unwrap()[..]).unwrap();

    App::build()
        .add_resource(WindowDescriptor {
            title: String::from("Dual Gravity"),
            width: 800,
            height: 500,
            resizable: false,
            ..Default::default()
        })
        .add_default_plugins()
        .add_resource(ClearColor(Color::BLACK))
        .add_resource(Boost(cfg.boost))
        .add_resource(Gravity(cfg.gravity))
        .add_startup_system(startup.system())
        .add_system(boost.system())
        .add_system(gravity.system())
        .add_system(movement.system())
        .add_system(render.system())
        .add_system(game_over.system())
        .run();
}
