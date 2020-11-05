use crate::types::{Ball, Boost, Gravity, Position, Velocity};

use bevy::{
    app::AppExit,
    input::{keyboard::KeyCode, Input},
    prelude::*,
};
use rand::{thread_rng, Rng};

pub fn startup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let mut rng = thread_rng();

    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(Color::WHITE.into()),
            sprite: Sprite {
                size: Vec2::new(20.0, 20.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Ball)
        .with(Position(rng.gen_range(-0.1, 0.1)))
        .with(Velocity(rng.gen_range(-0.01, 0.01)));
}

pub fn boost(
    input: Res<Input<KeyCode>>,
    boost: Res<Boost>,
    mut query: Query<(&Ball, &mut Velocity)>,
) {
    for (_, mut v) in query.iter_mut() {
        match (input.pressed(KeyCode::A), input.pressed(KeyCode::D)) {
            (true, false) => v.0 -= boost.0,
            (false, true) => v.0 += boost.0,
            _ => (),
        }
    }
}

pub fn gravity(g: Res<Gravity>, mut query: Query<(&Ball, &Position, &mut Velocity)>) {
    for (_, x, mut v) in query.iter_mut() {
        let r = x.0 + 1.25;
        v.0 -= g.0 / (r * r);

        let r = 1.25 - x.0;
        v.0 += g.0 / (r * r);
    }
}

pub fn movement(mut query: Query<(&Ball, &mut Position, &Velocity)>) {
    for (_, mut x, v) in query.iter_mut() {
        x.0 += v.0;
    }
}

pub fn render(mut query: Query<(&Ball, &Position, &mut Transform)>) {
    for (_, x, mut tf) in query.iter_mut() {
        tf.translation.set_x(x.0 * 400.0 - 10.0);
    }
}

pub fn game_over(
    time: Res<Time>,
    mut exit: ResMut<Events<AppExit>>,
    mut query: Query<(&Ball, &Position)>,
) {
    for (_, x) in query.iter_mut() {
        if x.0 <= -1.0 || x.0 >= 1.0 {
            println!("{} seconds", time.seconds_since_startup);
            exit.send(AppExit);
        }
    }
}
