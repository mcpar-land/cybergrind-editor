use std::ops::RangeInclusive;

use bevy::prelude::*;
use bevy_mod_raycast::{
	DefaultRaycastingPlugin, RayCastMesh, RayCastMethod, RayCastSource,
};
use bevy_prototype_debug_lines::DebugLines;

use crate::{
	map3d::{MapMaterials, MapResource, Pillar},
	util::is_inside_box,
};

pub struct SelectMaterials {
	pub hovered: Handle<StandardMaterial>,
	pub selected: Handle<StandardMaterial>,
	pub boxed: Handle<StandardMaterial>,
}

impl FromWorld for SelectMaterials {
	fn from_world(world: &mut World) -> Self {
		let mut materials = world
			.get_resource_mut::<Assets<StandardMaterial>>()
			.expect("Failed to get material asset");
		Self {
			hovered: materials.add(Color::CYAN.into()),
			selected: materials.add(Color::BLUE.into()),
			boxed: materials.add(Color::ALICE_BLUE.into()),
		}
	}
}

pub struct Selection {
	pub selections: [[bool; 16]; 16],
	pub box_select: Option<((usize, usize), (usize, usize))>,
}

impl FromWorld for Selection {
	fn from_world(_: &mut World) -> Self {
		Self {
			selections: [[false; 16]; 16],
			box_select: None,
		}
	}
}

#[derive(Default)]
pub struct Selectable {
	pub selected: bool,
	pub hovered: bool,
	pub boxed: bool,
}

impl Selectable {
	pub fn selected(&self) -> bool {
		self.selected
	}
}

pub struct SelectableRaycastSet;

fn raycast_screen_space(
	mut cursor: EventReader<CursorMoved>,
	mut query: Query<&mut RayCastSource<SelectableRaycastSet>>,
) {
	for mut source in &mut query.iter_mut() {
		if let Some(mouse) = cursor.iter().last() {
			source.cast_method = RayCastMethod::Screenspace(mouse.position);
		}
	}
}

fn raycast_handle_mouse(
	// mut lines: ResMut<DebugLines>,
	mut selection: ResMut<Selection>,
	mouse_button_input: Res<Input<MouseButton>>,
	keyboard_input: Res<Input<KeyCode>>,
	map: Res<MapResource>,
	pick_source_query: Query<&RayCastSource<SelectableRaycastSet>>,
	mut query: Query<
		(&Pillar, &mut Selectable, Entity),
		With<RayCastMesh<SelectableRaycastSet>>,
	>,
) {
	let mouse_released = mouse_button_input.just_released(MouseButton::Left);
	let shift = keyboard_input.pressed(KeyCode::LShift);
	let hover_offset = Vec3::new(0.25, 0.1, 0.25);
	for (_, mut selectable, _) in query.iter_mut() {
		selectable.hovered = false;
	}
	for pick_source in pick_source_query.iter() {
		if let Some((entity, _)) = pick_source.intersect_top() {
			if let Ok((Pillar(x, y), mut selectable, _)) = query.get_mut(entity) {
				let h = map.0.heights.get(*x, *y).unwrap().0;
				// let center =
				// 	Vec3::new(*x as f32, h as f32, *y as f32) - Vec3::splat(7.5);
				// let offset = Vec3::new(1.0, 16.0, 1.0);
				// draw_cube(
				// 	&mut lines,
				// 	center + offset / 2.0 + hover_offset,
				// 	center - offset / 2.0 - hover_offset,
				// 	Color::PINK,
				// );
				selectable.hovered = true;

				if mouse_button_input.just_pressed(MouseButton::Left) {
					selection.box_select = Some(((*x, *y), (*x, *y)));
				} else if mouse_button_input.pressed(MouseButton::Left) {
					if let Some(box_select) = &mut selection.box_select {
						box_select.1 = (*x, *y);
					}
				} else if !mouse_released {
					selection.box_select = None;
				}
			}
		}
	}
	for (Pillar(x, y), mut selectable, _) in query.iter_mut() {
		if let Some(range) = &selection.box_select {
			// in_box, was_selected, shift, mouse_released
			let in_box = is_inside_box((*x, *y), range);
			let was_selected = selectable.selected();

			if mouse_released {
				if was_selected && !shift {
					selectable.selected = false;
				}
				if in_box {
					selectable.selected = true;
				}
			} else {
				if in_box {
					selectable.boxed = true;
				}
			}

			if is_inside_box((*x, *y), range) {
				if mouse_button_input.just_released(MouseButton::Left) {
					selectable.boxed = false;
					selectable.selected = true;
				} else {
					selectable.boxed = true;
				}
			} else {
				selectable.boxed = false;
			}
		} else {
			selectable.boxed = false;
		}
	}
}

pub fn draw_cube(lines: &mut DebugLines, a: Vec3, b: Vec3, color: Color) {
	let diff = a - b;

	let (a1, a2, a3) = (
		a - Vec3::X * diff.x,
		Vec3::new(b.x, a.y, b.z),
		a - Vec3::Z * diff.z,
	);

	let (b1, b2, b3) = (
		b + Vec3::Z * diff.z,
		Vec3::new(a.x, b.y, a.z),
		b + Vec3::X * diff.x,
	);

	lines.line_colored(a, a1, 0.0, color);
	lines.line_colored(a1, a2, 0.0, color);
	lines.line_colored(a2, a3, 0.0, color);
	lines.line_colored(a3, a, 0.0, color);

	lines.line_colored(b, b1, 0.0, color);
	lines.line_colored(b1, b2, 0.0, color);
	lines.line_colored(b2, b3, 0.0, color);
	lines.line_colored(b3, b, 0.0, color);

	lines.line_colored(a, b2, 0.0, color);
	lines.line_colored(a1, b1, 0.0, color);
	lines.line_colored(a3, b3, 0.0, color);
	lines.line_colored(a2, b, 0.0, color);
}

pub fn selection_material_switch(
	mats: Res<SelectMaterials>,
	map_mats: Res<MapMaterials>,
	mut query: Query<
		(&mut Handle<StandardMaterial>, &Selectable),
		Changed<Selectable>,
	>,
) {
	for (mut mat, sel) in query.iter_mut() {
		if sel.hovered {
			*mat = mats.hovered.clone();
		} else if sel.boxed {
			*mat = mats.boxed.clone();
		} else if sel.selected {
			*mat = mats.selected.clone();
		} else {
			*mat = map_mats.box_mat.clone();
		}
	}
}

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app
			.add_plugin(DefaultRaycastingPlugin::<SelectableRaycastSet>::default())
			.add_system_to_stage(CoreStage::PreUpdate, raycast_screen_space.system())
			.add_system_to_stage(CoreStage::Update, raycast_handle_mouse.system())
			.add_system_to_stage(
				CoreStage::PostUpdate,
				selection_material_switch.system(),
			)
			.init_resource::<Selection>()
			.init_resource::<SelectMaterials>();
	}
}
