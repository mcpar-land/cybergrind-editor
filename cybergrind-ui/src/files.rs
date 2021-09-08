use std::fs::{File, OpenOptions};
use std::io::{prelude::*, SeekFrom};

use bevy::prelude::*;
use cybergrind_core::{Map, Parsable};

use crate::map3d::MapResource;
pub struct LoadedFile {
	pub file: Option<(File, String)>,
	pub unsaved_changes: bool,
}

impl LoadedFile {
	pub fn window_title(&self) -> String {
		let filename = format!(
			"{}{}",
			if self.unsaved_changes { "*" } else { "" },
			if let Some((_, path)) = &self.file {
				&path
			} else {
				""
			}
		);
		format!(
			"Cybergrind Editor{}",
			if !filename.is_empty() {
				format!(" - {}", filename)
			} else {
				"".to_string()
			}
		)
	}
}

impl FromWorld for LoadedFile {
	fn from_world(_: &mut World) -> Self {
		Self {
			file: None,
			unsaved_changes: false,
		}
	}
}

#[derive(Clone)]
pub enum FileEvent {
	Open,
	Save,
	SaveAs,
	New,
}

fn file_event_handler_system(
	mut windows: ResMut<Windows>,
	mut map: ResMut<MapResource>,
	mut loaded_file: ResMut<LoadedFile>,
	mut ev_files: EventReader<FileEvent>,
) {
	fn open(loaded_file: &mut LoadedFile, map: &mut MapResource) {
		println!("File event open");

		if let nfd::Response::Okay(path) =
			nfd::dialog().open().expect("Error opening file dialog")
		{
			let file = match OpenOptions::new()
				.read(true)
				.write(true)
				.append(false)
				.open(&path)
			{
				Ok(mut file) => {
					let mut contents = String::new();
					if let Err(err) = file.read_to_string(&mut contents) {
						println!("Error reading file: {}", err);
					}

					map.0 = match Map::from_str(&contents) {
						Ok(map) => map,
						Err(err) => {
							println!("Error parsing map file: {}", err);
							return;
						}
					};

					file
				}
				Err(err) => {
					println!("Error opening file: {}", err);
					return;
				}
			};
			loaded_file.file = Some((file, path));
			loaded_file.unsaved_changes = false;
		}
	}
	fn new(loaded_file: &mut LoadedFile, map: &mut MapResource) {
		println!("File event new");
		map.0 = Map::default();
		loaded_file.file = None;
		loaded_file.unsaved_changes = false;
	}

	fn save(loaded_file: &mut LoadedFile, map: &mut MapResource) {
		println!("File event save");
		if let Some((file, _)) = &mut loaded_file.file {
			if let Err(error) = file
				.seek(SeekFrom::Start(0))
				.and_then(|_| file.write(map.0.to_string().as_bytes()))
			{
				println!("Error saving file: {}", error);
			} else {
				println!("Saved file!");
				loaded_file.unsaved_changes = false;
			}
		} else {
			println!("No file open!");
			save_as(loaded_file, map);
		}
	}

	fn save_as(loaded_file: &mut LoadedFile, map: &mut MapResource) {
		println!("File event save as");
		if let nfd::Response::Okay(path) =
			nfd::open_save_dialog(None, None).expect("Error opening file dialog")
		{
			let file = match File::create(path.clone()) {
				Ok(file) => file,
				Err(err) => {
					println!("Error saving file as: {}", err);
					return;
				}
			};

			loaded_file.file = Some((file, path));
			loaded_file.unsaved_changes = false;
			save(loaded_file, map);
		}
	}

	for event in ev_files.iter() {
		let h = match event {
			FileEvent::Open => open,
			FileEvent::Save => save,
			FileEvent::SaveAs => save_as,
			FileEvent::New => new,
		};
		h(&mut loaded_file, &mut map);
		if let Some(win) = windows.get_primary_mut() {
			win.set_title(loaded_file.window_title());
		}
	}
}

pub fn files_system_set() -> SystemSet {
	SystemSet::new().with_system(file_event_handler_system.system())
}
