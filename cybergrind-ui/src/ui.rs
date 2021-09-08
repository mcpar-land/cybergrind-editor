use bevy::prelude::*;

use crate::{
	files::{FileEvent, LoadedFile},
	ui::dialog::{Dialog, DialogButton, DialogDispatch},
};

use self::dialog::setup_dialog;

pub mod dialog;

static HELP_TEXT: &'static str = r#"Q: None
W: Melee
E: Projectile
R: Stairs
T: Hideous

Ctrl + Z: Undo
Ctrl + N: New
Ctrl + S: Save
Ctrl + A: Save As

Alt: Rotate
Alt + Scroll: Zoom"#;

pub struct ButtonMaterials {
	normal: Handle<ColorMaterial>,
	hovered: Handle<ColorMaterial>,
	pressed: Handle<ColorMaterial>,
	font: Handle<Font>,
}

impl FromWorld for ButtonMaterials {
	fn from_world(world: &mut World) -> Self {
		let (normal, hovered, pressed) = {
			let mut materials =
				world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
			(
				materials.add(Color::BLACK.into()),
				materials.add(Color::DARK_GRAY.into()),
				materials.add(Color::WHITE.into()),
			)
		};

		let mut fonts = world.get_resource_mut::<Assets<Font>>().unwrap();

		ButtonMaterials {
			normal,
			hovered,
			pressed,
			font: fonts.add(
				Font::try_from_bytes(
					include_bytes!("../assets/FiraMono-Medium.ttf").to_vec(),
				)
				.expect("Could not load font"),
			),
		}
	}
}

#[derive(Clone)]
pub struct MenuButton {
	pub kind: MenuButtonKind,
	pub name: &'static str,
	pub key_code: KeyCode,
}

#[derive(Clone)]
pub enum MenuButtonKind {
	New,
	Open,
	Save,
	SaveAs,
}

const MENU_BUTTONS: [MenuButton; 4] = [
	MenuButton {
		kind: MenuButtonKind::New,
		name: "(N)ew",
		key_code: KeyCode::N,
	},
	MenuButton {
		kind: MenuButtonKind::Open,
		name: "(O)pen",
		key_code: KeyCode::O,
	},
	MenuButton {
		kind: MenuButtonKind::Save,
		name: "(S)ave",
		key_code: KeyCode::S,
	},
	MenuButton {
		kind: MenuButtonKind::SaveAs,
		name: "Save (A)s",
		key_code: KeyCode::A,
	},
];

pub fn setup_ui(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut windows: ResMut<Windows>,
	loaded_file: Res<LoadedFile>,
	button_materials: Res<ButtonMaterials>,
) {
	commands.spawn_bundle(UiCameraBundle::default());

	if let Some(win) = windows.get_primary_mut() {
		win.set_title(loaded_file.window_title());
	}

	// Top bar menu
	commands
		.spawn_bundle(NodeBundle {
			style: Style {
				size: Size::new(Val::Percent(100.0), Val::Px(15.0)),
				position_type: PositionType::Absolute,
				border: Rect::all(Val::Px(1.0)),
				position: Rect {
					left: Val::Px(0.0),
					top: Val::Px(0.0),
					bottom: Val::Undefined,
					right: Val::Undefined,
				},
				display: Display::Flex,
				..Default::default()
			},
			material: materials.add(Color::BLACK.into()),
			..Default::default()
		})
		.with_children(|parent| {
			for menu_button in MENU_BUTTONS.iter() {
				parent
					.spawn_bundle(ButtonBundle {
						style: Style {
							size: Size::new(Val::Auto, Val::Px(13.0)),
							margin: Rect {
								right: Val::Px(1.0),
								..Default::default()
							},
							padding: Rect {
								right: Val::Px(7.0),
								left: Val::Px(7.0),
								..Default::default()
							},
							..Default::default()
						},
						material: button_materials.normal.clone(),
						..Default::default()
					})
					.insert(menu_button.clone())
					.with_children(|parent| {
						parent.spawn_bundle(TextBundle {
							text: Text::with_section(
								menu_button.name.to_string(),
								TextStyle {
									font_size: 10.0,
									color: Color::WHITE,
									font: button_materials.font.clone(),
								},
								Default::default(),
							),
							..Default::default()
						});
					});
			}
		});

	// qwer display
	commands
		.spawn_bundle(NodeBundle {
			style: Style {
				size: Size::new(Val::Auto, Val::Auto),
				position_type: PositionType::Absolute,
				border: Rect::all(Val::Px(5.0)),
				position: Rect {
					left: Val::Px(15.0),
					top: Val::Px(15.0 + 15.0),
					..Default::default()
				},
				display: Display::Flex,
				..Default::default()
			},
			material: materials.add(
				Color::Rgba {
					red: 0.0,
					green: 0.0,
					blue: 0.0,
					alpha: 0.5,
				}
				.into(),
			),
			..Default::default()
		})
		.with_children(|parent| {
			parent.spawn_bundle(TextBundle {
				text: Text::with_section(
					HELP_TEXT,
					TextStyle {
						font_size: 10.0,
						color: Color::WHITE,
						font: button_materials.font.clone(),
					},
					Default::default(),
				),
				..Default::default()
			});
		});

	setup_dialog(commands);
}

fn update_button_color(
	button_materials: &ButtonMaterials,
	interaction: Interaction,
) -> Handle<ColorMaterial> {
	match interaction {
		Interaction::Clicked => button_materials.pressed.clone(),
		Interaction::Hovered => button_materials.hovered.clone(),
		Interaction::None => button_materials.normal.clone(),
	}
}

pub fn button_color_system(
	button_materials: Res<ButtonMaterials>,
	mut interaction_query: Query<
		(&Interaction, &mut Handle<ColorMaterial>),
		(Changed<Interaction>, With<Button>),
	>,
) {
	for (interaction, mut material) in interaction_query.iter_mut() {
		*material = update_button_color(&button_materials, *interaction);
	}
}

pub fn menu_button_click_system(
	query: Query<(&Interaction, &MenuButton), Changed<Interaction>>,
	mut ev_menu_button: EventWriter<MenuButtonKind>,
) {
	for (interaction, menu_button) in query.iter() {
		if interaction == &Interaction::Clicked {
			ev_menu_button.send(menu_button.kind.clone());
		}
	}
}

pub fn menu_button_shortcut_system(
	key: Res<Input<KeyCode>>,
	windows: Res<Windows>,
	mut ev_menu_button: EventWriter<MenuButtonKind>,
) {
	let win = windows.get_primary().expect("no primary window");
	if win.is_focused() {
		for menu_button in MENU_BUTTONS.iter() {
			if key.just_pressed(menu_button.key_code)
				&& key.pressed(KeyCode::LControl)
			{
				ev_menu_button.send(menu_button.kind.clone());
			}
		}
	}
}

fn menu_button_handler_system(
	mut ev_menu_button: EventReader<MenuButtonKind>,
	mut ev_files: EventWriter<FileEvent>,
	loaded_file: Res<LoadedFile>,
	mut dialog: ResMut<Dialog>,
) {
	fn dialog_unsaved(action: &str, button: &str, dispatch: FileEvent) -> Dialog {
		Dialog {
			active: true,
			title: format!("Unsaved Changes"),
			text: format!(
				"You have unsaved changes! Are you sure you want to {}?",
				action
			),
			button_left: DialogButton {
				text: format!("{}", button),
				dispatch: DialogDispatch::File(dispatch),
			},
			button_right: Some(DialogButton {
				text: "Cancel".to_string(),
				dispatch: DialogDispatch::Close,
			}),
		}
	}

	for ev in ev_menu_button.iter() {
		if loaded_file.unsaved_changes {
			match ev {
				MenuButtonKind::New => {
					*dialog = dialog_unsaved("New", "create a new file", FileEvent::New);
				}
				MenuButtonKind::Open => {
					*dialog =
						dialog_unsaved("Open", "open another file", FileEvent::Open);
				}
				MenuButtonKind::Save => {
					ev_files.send(FileEvent::Save);
				}
				MenuButtonKind::SaveAs => {
					ev_files.send(FileEvent::SaveAs);
				}
			};
		} else {
			ev_files.send(match ev {
				MenuButtonKind::Open => FileEvent::Open,
				MenuButtonKind::Save => FileEvent::Save,
				MenuButtonKind::New => FileEvent::New,
				MenuButtonKind::SaveAs => FileEvent::SaveAs,
			})
		}
	}
}

pub fn ui_system_set() -> SystemSet {
	SystemSet::new()
		.with_system(button_color_system.system())
		.with_system(menu_button_click_system.system())
		.with_system(menu_button_shortcut_system.system())
		.with_system(menu_button_handler_system.system())
}
