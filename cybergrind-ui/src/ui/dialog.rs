use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::files::FileEvent;

#[derive(Default)]
pub struct Dialog {
	pub active: bool,
	pub title: String,
	pub text: String,
	pub button_left: DialogButton,
	pub button_right: Option<DialogButton>,
}

#[derive(Default)]
pub struct DialogButton {
	pub text: String,
	pub dispatch: DialogDispatch,
}

#[derive(Clone)]
pub enum DialogDispatch {
	Close,
	File(FileEvent),
}

impl Dialog {
	pub fn close(&mut self) {
		*self = Dialog {
			active: false,
			title: "Dialog".to_string(),
			text: "This shouldn't appear".to_string(),
			button_left: DialogButton {
				text: "OK".to_string(),
				dispatch: DialogDispatch::Close,
			},
			button_right: None,
		};
	}
}

impl Default for DialogDispatch {
	fn default() -> Self {
		Self::Close
	}
}
pub fn setup_dialog(mut commands: Commands) {
	commands.insert_resource(Dialog::default());
}

pub fn dialog_system(
	egui_ctx: Res<EguiContext>,
	mut dialog: ResMut<Dialog>,
	mut ev_dialog: EventWriter<DialogDispatch>,
) {
	if dialog.active {
		egui::Window::new(&dialog.title)
			.collapsible(false)
			.resizable(false)
			.anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
			.show(egui_ctx.ctx(), |ui| {
				ui.vertical_centered(|ui| {
					ui.label(&dialog.text);
					ui.horizontal(|ui| {
						if ui.button(&dialog.button_left.text).clicked() {
							ev_dialog.send(dialog.button_left.dispatch.clone());
							dialog.close();
						}
						if let Some(right) = &dialog.button_right {
							if ui.button(&right.text).clicked() {
								ev_dialog.send(right.dispatch.clone());
								dialog.close();
							}
						}
						ui.shrink_width_to_current();
					});
				})
			});
	}
}

pub fn dialog_event_conversion_system(
	mut dialog: ResMut<Dialog>,
	mut ev_dialog: EventReader<DialogDispatch>,
	mut ev_files: EventWriter<FileEvent>,
) {
	for event in ev_dialog.iter() {
		match event {
			DialogDispatch::Close => {
				dialog.close();
			}
			DialogDispatch::File(file_event) => {
				ev_files.send(file_event.clone());
			}
		}
	}
}

pub fn dialog_system_set() -> SystemSet {
	SystemSet::new()
		.with_system(dialog_event_conversion_system.system())
		.with_system(dialog_system.system())
}
