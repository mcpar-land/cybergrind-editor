use bevy::{pbr::AmbientLight, prelude::*};
use bevy_egui::{EguiContext, EguiPlugin};
use bevy_mod_picking::*;
use bevy_obj::ObjPlugin;
use bevy_prototype_debug_lines::*;
use controls::{
	controls_system_set, cursor_loop_system, prefab_edit, scroll_edit,
};
use cybergrind_core::{Map, Parsable};
use files::{files_system_set, FileEvent, LoadedFile};
use grid::draw_grid;
use history::HistoryPlugin;
use map3d::{spawn_map, update_map_display, update_prefabs, MapResource};
use smooth_bevy_cameras::{
	controllers::orbit::{
		OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin,
	},
	LookTransformPlugin,
};
use ui::{
	dialog::{dialog_system_set, DialogDispatch},
	setup_ui, ui_system_set, ButtonMaterials, MenuButtonKind,
};

mod controls;
mod files;
mod grid;
mod history;
mod map3d;
mod pillar_mesh;
mod ui;

fn setup(mut commands: Commands, mut ambient_light: ResMut<AmbientLight>) {
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
		.add_plugin(EguiPlugin)
		.insert_resource(DebugLines {
			depth_test: true,
			..Default::default()
		})
		.add_plugin(ObjPlugin)
		.add_plugin(LookTransformPlugin)
		.add_plugin(OrbitCameraPlugin)
		.add_plugin(PickingPlugin)
		.add_plugin(InteractablePickingPlugin)
		.add_plugin(HighlightablePickingPlugin)
		.add_plugin(HistoryPlugin)
		.init_resource::<LoadedFile>()
		.init_resource::<ButtonMaterials>()
		.add_startup_system(setup.system())
		.add_startup_system(spawn_map.system())
		.add_startup_system(setup_ui.system())
		.add_event::<MenuButtonKind>()
		.add_event::<FileEvent>()
		.add_event::<DialogDispatch>()
		.insert_resource(MapResource(Map::default()))
		.add_system(update_map_display.system())
		.add_system(update_prefabs.system())
		.add_system(draw_grid.system())
		.add_system_set(ui_system_set())
		.add_system_set(dialog_system_set())
		.add_system_set(files_system_set())
		.add_system_set(controls_system_set())
		.run();
}
