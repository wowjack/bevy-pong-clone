use crate::ball::Ball;
use crate::score::Score;
use crate::{Player, Reset};
use bevy::prelude::*;
use bevy::ecs::prelude::Res;
use bevy::ecs::system::{Commands, Query, ResMut};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::Windows;
use bevy::sprite::collide_aabb::collide;
use bevy::sprite::Sprite;
use bevy::transform::components::Transform;
use std::ops::DerefMut;

#[derive(Component)]
pub struct Goal;

impl Goal {
	const THICKNESS: f32 = 20.0;
}

pub fn spawn_goals(commands: &mut Commands) {
	spawn_goal(commands, Player::Left);
	spawn_goal(commands, Player::Right);
}

fn spawn_goal(commands: &mut Commands, player: Player) {
	commands
		.spawn()
		.insert_bundle(SpriteBundle::default())
		.insert(Goal)
		.insert(player);
}

pub fn goal_reset_system(
	mut reset_reader: EventReader<Reset>,
	windows: Res<Windows>,
	mut query: Query<(&mut Sprite, &mut Transform, &Goal, &Player)>,
) {
	if reset_reader.iter().last().is_none() {
		return;
	}

	let window = windows.get_primary().unwrap();

	for (_sprite, mut transform, _goal, player) in query.iter_mut() {
		transform.scale = Vec3::new(Goal::THICKNESS, window.height(), 1.);

		use Player::*;
		let x_offset = (window.width() - Goal::THICKNESS) / 2.0;
		let x_position = match player {
			Left => x_offset,
			Right => -x_offset,
		};
		transform.translation = Vec3::new(x_position, 0.0, 0.0);
	}
}

pub fn goal_collision_system(
	mut reset_writer: EventWriter<Reset>,
	ball_query: Query<(&Ball, &Transform, &Sprite)>,
	goal_query: Query<(&Transform, &Sprite, &Goal, &Player)>,
	mut score: ResMut<Score>,
) {
	for (_ball, ball_transform, _ball_sprite) in ball_query.iter() {
		for (goal_transform, _goal_sprite, _goal, player) in goal_query.iter() {
			let collision = collide(
				ball_transform.translation,
				Vec2::new(ball_transform.scale.x, ball_transform.scale.y),
				goal_transform.translation,
				Vec2::new(goal_transform.scale.x, goal_transform.scale.y),
			);

			if collision.is_some() {
				use Player::*;
				match player {
					Left => score.deref_mut().left += 1,
					Right => score.deref_mut().right += 1,
				}
				reset_writer.send(Reset);
			}
		}
	}
}
