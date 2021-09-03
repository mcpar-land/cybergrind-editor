use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_mod_picking::Selection;
use cybergrind_core::Prefab;

use crate::{
	history::{Edit, EditData},
	map3d::{MapResource, Pillar},
};

pub fn cursor_loop_system(
	key: Res<Input<KeyCode>>,
	mut windows: ResMut<Windows>,
) {
	let win = windows.get_primary_mut().expect("no primary window");
	if key.pressed(KeyCode::LAlt) {
		win.set_cursor_lock_mode(true);
		let mouse_pos = win.cursor_position();
		let win_height = win.physical_height() as f32;
		let win_width = win.physical_width() as f32;
		if let Some(mouse_pos) = mouse_pos {
			if mouse_pos.x <= 0.0 {
				win.set_cursor_position(Vec2::new(win_width, mouse_pos.y))
			} else if mouse_pos.x >= win_width - 1.0 {
				win.set_cursor_position(Vec2::new(0.0, mouse_pos.y))
			} else if mouse_pos.y <= 1.0 {
				win.set_cursor_position(Vec2::new(mouse_pos.x, win_height))
			} else if mouse_pos.y >= win_height {
				win.set_cursor_position(Vec2::new(mouse_pos.x, 0.0))
			}
		}
	} else {
		win.set_cursor_lock_mode(false);
	}
}

pub fn scroll_edit(
	key: Res<Input<KeyCode>>,
	mut mouse_wheel_events: EventReader<MouseWheel>,
	// mut map: ResMut<MapResource>,
	mut edit_events: EventWriter<Edit>,
	query: Query<(&Selection, &Pillar)>,
) {
	if !key.pressed(KeyCode::LAlt) {
		for event in mouse_wheel_events.iter() {
			let move_delta: i8 = if event.y > 0.0 {
				1
			} else if event.y < 0.0 {
				-1
			} else {
				0
			};

			let squares = query
				.iter()
				.filter(|(s, _)| s.selected())
				.map(|(_, Pillar(x, y))| (*x, *y))
				.collect::<Vec<(usize, usize)>>();

			edit_events.send(Edit {
				data: EditData::Height(move_delta),
				squares,
			});
		}
	}
}

pub fn prefab_edit(
	key: Res<Input<KeyCode>>,
	mut edit_events: EventWriter<Edit>,
	map: Res<MapResource>,
	query: Query<(&Selection, &Pillar)>,
) {
	for pressed in key.get_just_pressed() {
		let prefab = match pressed {
			KeyCode::Q => Prefab::None,
			KeyCode::W => Prefab::Melee,
			KeyCode::E => Prefab::Projectile,
			KeyCode::R => Prefab::Stairs,
			KeyCode::T => Prefab::Hideous,
			_ => {
				return;
			}
		};
		println!("Button press for setting prefab {:?}", prefab);

		let (squares, from): (Vec<(usize, usize)>, Vec<Prefab>) = query
			.iter()
			.filter(|(s, _)| s.selected())
			.map(|(_, Pillar(x, y))| {
				((*x, *y), map.0.prefabs.get(*x, *y).unwrap().clone())
			})
			.unzip();

		edit_events.send(Edit {
			data: EditData::Prefab { from, to: prefab },
			squares,
		});
	}
}
