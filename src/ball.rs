use crate::{Collider, Reset};
use bevy::prelude::*;
use bevy::core::Time;
use bevy::ecs::system::{Commands, Query, Res};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::Windows;
use bevy::sprite::collide_aabb::collide;
use bevy::sprite::collide_aabb::Collision;
use bevy::sprite::Sprite;
use bevy::transform::components::Transform;

#[derive(Component)]
pub struct Ball {
	speed: f32,
	direction: Vec2,
}

impl Ball {
	pub fn velocity(&self) -> Vec2 {
		self.speed * self.direction.normalize()
	}
}

impl Default for Ball {
	fn default() -> Self {
		Self {
			speed: Default::default(),
			direction: Vec2::new(1.0, 1.0).normalize(),
		}
	}
}

pub fn spawn_ball(commands: &mut Commands) {
	commands
		.spawn()
		.insert_bundle(SpriteBundle::default())
		.insert(Ball::default());
}

pub fn ball_reset_system(
	mut reset_reader: EventReader<Reset>,
	windows: Res<Windows>,
	mut query: Query<(&mut Sprite, &mut Transform, &mut Ball)>,
) {
	if reset_reader.iter().last().is_none() {
		return;
	}

	let window = windows.get_primary().unwrap();

	for (_sprite, mut transform, mut ball) in query.iter_mut() {
		ball.speed = window.height() / 1.5;

		let ball_width = 0.05 * window.height();
		transform.scale = Vec3::new(ball_width, ball_width, 1.);

		transform.translation = Vec3::default();
	}
}

pub fn ball_movement_system(time: Res<Time>, mut query: Query<(&Ball, &mut Transform)>) {
	let time_delta = time.delta_seconds();
	for (ball, mut transform) in query.iter_mut() {
		transform.translation += time_delta * ball.velocity().extend(0.0);
	}
}

pub fn ball_collision_system(
	mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
	collider_query: Query<(&Collider, &Transform, &Sprite)>,
) {
	for (mut ball, ball_transform, _ball_sprite) in ball_query.iter_mut() {
		for (_collider, collider_transform, _collider_sprite) in collider_query.iter() {
			let collision = collide(
				ball_transform.translation,
				Vec2::new(ball_transform.scale.x, ball_transform.scale.y),
				collider_transform.translation,
				Vec2::new(collider_transform.scale.x, collider_transform.scale.y),
			);

			let collision = match collision {
				Some(collision) => collision,
				None => continue,
			};

			use Collision::*;
			let (reflect_x, reflect_y) = match collision {
				Left => (ball.direction.x > 0.0, false),
				Right => (ball.direction.x < 0.0, false),
				Top => (false, ball.direction.y < 0.0),
				Bottom => (false, ball.direction.y > 0.0),
				_ => (false, false)
			};

			if reflect_x {
				ball.direction.x = -ball.direction.x;
			}

			if reflect_y {
				ball.direction.y = -ball.direction.y;
			}
		}
	}
}
