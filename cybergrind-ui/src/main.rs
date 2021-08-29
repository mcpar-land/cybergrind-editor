use bevy::{pbr::AmbientLight, prelude::*};
use bevy_mod_picking::*;
use bevy_prototype_debug_lines::*;
use controls::cursor_loop_system;
use cybergrind_core::Map;
use grid::draw_grid;
use map3d::{spawn_map, update_map_display, MapResource};
use smooth_bevy_cameras::{
	controllers::orbit::{
		OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin,
	},
	LookTransformPlugin,
};

mod controls;
mod grid;
mod map3d;

const TEST_MAP: &'static str = include_str!("../test2.gcp");

fn setup(mut commands: Commands, mut ambient_light: ResMut<AmbientLight>) {
	commands.spawn_bundle(UiCameraBundle::default());
	ambient_light.color = Color::WHITE;
	ambient_light.brightness = 1.0;
	commands.spawn_bundle(LightBundle {
		transform: Transform::from_translation(Vec3::new(32.0, -32.0, 32.0)),
		..Default::default()
	});

	commands
		.spawn_bundle(OrbitCameraBundle::new(
			OrbitCameraController {
				mouse_rotate_sensitivity: Vec2::splat(0.003),
				mouse_translate_sensitivity: Vec2::ZERO,
				mouse_wheel_zoom_sensitivity: 0.0,
				..Default::default()
			},
			PerspectiveCameraBundle::default(),
			Vec3::new(-16.0, 16.0, -16.0),
			Vec3::new(0.0, 0.0, 0.0),
		))
		.insert_bundle(PickingCameraBundle::default());
}

fn main() {
	App::build()
		.insert_resource(Msaa { samples: 4 })
		.add_plugins(DefaultPlugins)
		.add_plugin(DebugLinesPlugin)
		.insert_resource(DebugLines {
			depth_test: true,
			..Default::default()
		})
		.add_plugin(LookTransformPlugin)
		.add_plugin(OrbitCameraPlugin)
		.add_plugin(PickingPlugin)
		.add_plugin(InteractablePickingPlugin)
		.add_plugin(HighlightablePickingPlugin)
		.add_startup_system(setup.system())
		.add_startup_system(spawn_map.system())
		.insert_resource(MapResource(Map::from_str(TEST_MAP).unwrap()))
		.add_system(update_map_display.system())
		.add_system(cursor_loop_system.system())
		.add_system(draw_grid.system())
		.run();
}
