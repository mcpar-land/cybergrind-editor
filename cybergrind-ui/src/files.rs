use std::fs::{File, OpenOptions};
use std::io::{prelude::*, SeekFrom};

use bevy::prelude::*;
use cybergrind_core::Map;

use crate::map3d::MapResource;
pub struct LoadedFile(pub Option<File>);

impl FromWorld for LoadedFile {
	fn from_world(_: &mut World) -> Self {
		Self(None)
	}
}

pub enum FileEvent {
	Open(String),
	Save,
	SaveAs(String),
	New,
}

fn file_event_handler_system(
	mut map: ResMut<MapResource>,
	mut loaded_file: ResMut<LoadedFile>,
	mut ev_files: EventReader<FileEvent>,
) {
	for event in ev_files.iter() {
		match event {
			FileEvent::Open(path) => {
				println!("File event open");
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
								break;
							}
						};

						file
					}
					Err(err) => {
						println!("Error opening file: {}", err);
						break;
					}
				};
				loaded_file.0 = Some(file);
			}
			FileEvent::Save => {
				println!("File event save");
				if let Some(file) = &mut loaded_file.0 {
					if let Err(error) = file
						.seek(SeekFrom::Start(0))
						.and_then(|_| file.write(map.0.to_string().as_bytes()))
					{
						println!("Error saving file: {}", error);
					} else {
						println!("Saved file!");
					}
				} else {
					println!("No file open!");
				}
			}
			FileEvent::SaveAs(path) => {
				println!("File event save as");
			}
			FileEvent::New => {
				println!("File event new");
			}
		}
	}
}

pub fn files_system_set() -> SystemSet {
	SystemSet::new().with_system(file_event_handler_system.system())
}
