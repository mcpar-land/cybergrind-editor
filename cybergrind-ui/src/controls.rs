use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_mod_picking::Selection;

use crate::map3d::{MapResource, Pillar};

pub fn cursor_loop_system(
	key: Res<Input<KeyCode>>,
	mut windows: ResMut<Windows>,
) {
	let win = windows.get_primary_mut().expect("no primary window");
	if key.pressed(KeyCode::LControl) {
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
	mut mouse_wheel_events: EventReader<MouseWheel>,
	mut map: ResMut<MapResource>,
	mut query: Query<(&Selection, &Pillar)>,
) {
	for event in mouse_wheel_events.iter() {
		let move_delta: i8 = if event.y > 0.0 {
			1
		} else if event.y < 0.0 {
			-1
		} else {
			0
		};

		for (selection, Pillar(x, y)) in query.iter() {
			if selection.selected() {
				map.0.heights.0[*y][*x].0 += move_delta;
			}
		}
	}
}
