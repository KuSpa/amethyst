use crate::systems::{BounceSystem, MoveBallsSystem, PaddleSystem, WinnerSystem};
use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::prelude::DispatcherBuilder,
};

/// A bundle is a convenient way to initialise related resources, components and systems in a
/// world. This bundle prepares the world for a game of pong.
#[derive(Default)]
pub struct PongBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for PongBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(PaddleSystem, "paddle_system", &["input_system"]);
        builder.add(MoveBallsSystem, "ball_system", &[]);
        builder.add(
            BounceSystem,
            "collision_system",
            &["paddle_system", "ball_system"],
        );
        builder.add(
            WinnerSystem,
            "winner_system",
            &["paddle_system", "ball_system"],
        );
        Ok(())
    }
}
