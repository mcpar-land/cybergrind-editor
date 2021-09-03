use bevy::prelude::*;
use cybergrind_core::{Map, Prefab};

use crate::map3d::MapResource;

#[derive(Clone, Debug)]
pub enum EditData {
	Height(i8),
	Prefab { from: Vec<Prefab>, to: Prefab },
}

#[derive(Clone, Debug)]
pub struct Edit {
	pub data: EditData,
	pub squares: Vec<(usize, usize)>,
}

impl Edit {
	pub fn apply(&self, map: &mut Map) -> bool {
		match &self.data {
			EditData::Height(h) => {
				if *h == 0i8 {
					false
				} else {
					for (x, y) in self.squares.iter() {
						if let Some(height) = map.heights.get_mut(*x, *y) {
							height.0 = (height.0 + h).clamp(-50, 50);
						}
					}
					true
				}
			}
			EditData::Prefab { from, to } => {
				if from.iter().all(|f| *f == *to) {
					false
				} else {
					for (x, y) in self.squares.iter() {
						if let Some(prefab) = map.prefabs.get_mut(*x, *y) {
							*prefab = *to;
						}
					}
					true
				}
			}
		}
	}

	pub fn undo(&self, map: &mut Map) {
		for (i, (x, y)) in self.squares.iter().enumerate() {
			match &self.data {
				EditData::Height(h) => {
					if let Some(height) = map.heights.get_mut(*x, *y) {
						height.0 = (height.0 - h).clamp(-50, 50);
					}
				}
				EditData::Prefab { from, .. } => {
					if let Some(prefab) = map.prefabs.get_mut(*x, *y) {
						*prefab = from[i];
					}
				}
			}
		}
	}
}

static MAX_UNDO_HISTORY: usize = 500;

pub struct HistoryStack {
	pub stack: Vec<Edit>,
	pub pos: usize,
}

impl HistoryStack {
	pub fn push(&mut self, edit: Edit, map: &mut Map) {
		if edit.apply(map) {
			if self.stack.len() >= MAX_UNDO_HISTORY {
				self.stack.remove(0);
			}
			self.stack.push(edit);
		// println!("Pushed to history (len is now: {})", self.stack.len());
		} else {
			// println!("Tried to do a redundant edit, no history pushed");
		}
	}
	pub fn pop(&mut self, map: &mut Map) {
		if let Some(pop) = self.stack.pop() {
			pop.undo(map);
		}
	}
}

fn edit_with_history(
	mut edit_events: EventReader<Edit>,
	mut map: ResMut<MapResource>,
	mut history: ResMut<HistoryStack>,
) {
	for event in edit_events.iter() {
		history.push(event.clone(), &mut map.0);
	}
}

fn undo_handler(
	mut history: ResMut<HistoryStack>,
	mut map: ResMut<MapResource>,
	key: Res<Input<KeyCode>>,
) {
	if key.pressed(KeyCode::LControl) && key.just_pressed(KeyCode::Z) {
		history.pop(&mut map.0);
	}
}

pub struct HistoryPlugin;

impl Plugin for HistoryPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app
			.insert_resource(HistoryStack {
				stack: Vec::new(),
				pos: 0,
			})
			.add_event::<Edit>()
			.add_system(edit_with_history.system())
			.add_system(undo_handler.system());
	}
}
