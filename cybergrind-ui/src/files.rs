use bevy::prelude::*;
pub struct LoadedFile(pub Option<String>);

pub enum FileEvent {
	Open(String),
	Save,
	SaveAs(String),
	New,
}

impl FromWorld for LoadedFile {
	fn from_world(_: &mut World) -> Self {
		Self(None)
	}
}

fn file_event_handler_system(mut ev_files: EventReader<FileEvent>) {
	for event in ev_files.iter() {
		match event {
			FileEvent::Open(path) => {
				println!("File event open");
			}
			FileEvent::Save => {
				println!("File event save");
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
