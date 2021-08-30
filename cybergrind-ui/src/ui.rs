use bevy::prelude::*;

use crate::files::FileEvent;

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
	button_materials: Res<ButtonMaterials>,
) {
	commands.spawn_bundle(UiCameraBundle::default());

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
			if key.just_pressed(menu_button.key_code) {
				ev_menu_button.send(menu_button.kind.clone());
			}
		}
	}
}

fn menu_button_handler_system(
	mut ev_menu_button: EventReader<MenuButtonKind>,
	mut ev_files: EventWriter<FileEvent>,
) {
	for ev in ev_menu_button.iter() {
		match ev {
			MenuButtonKind::Open => {
				let res =
					nfd::open_file_dialog(None, None).expect("Error opening file dialog");
				if let nfd::Response::Okay(res) = res {
					ev_files.send(FileEvent::Open(res));
				}
			}
			MenuButtonKind::Save => {
				ev_files.send(FileEvent::Save);
			}
			MenuButtonKind::New => {
				ev_files.send(FileEvent::New);
			}
			MenuButtonKind::SaveAs => {
				let res =
					nfd::open_file_dialog(None, None).expect("Error opening file dialog");
				if let nfd::Response::Okay(res) = res {
					ev_files.send(FileEvent::SaveAs(res));
				}
			}
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
