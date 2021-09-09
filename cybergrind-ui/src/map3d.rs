use bevy::{prelude::*, render::texture::ImageType};
use bevy_mod_raycast::RayCastMesh;
use cybergrind_core::{Map, Prefab};

use crate::selection::{Selectable, SelectableRaycastSet};

pub const BOX_SCALE: f32 = 1.0;

pub struct MapResource(pub cybergrind_core::Map);

pub struct MapMaterials {
	pub box_mat: Handle<StandardMaterial>,
	pub box_mesh: Handle<Mesh>,
}

pub fn spawn_map(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut textures: ResMut<Assets<Texture>>,
	mut atlases: ResMut<Assets<TextureAtlas>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let box_mesh = meshes.add(Mesh::from(shape::Box::new(1.0, 16.0, 1.0)));
	let box_texture = Texture::from_buffer(
		include_bytes!("../assets/grid.png"),
		ImageType::Extension("png"),
	)
	.unwrap();
	let box_material = materials.add(textures.add(box_texture).into());

	let prefabs_texture = textures.add(
		Texture::from_buffer(
			include_bytes!("../assets/prefabs.png"),
			ImageType::Extension("png"),
		)
		.unwrap(),
	);

	let prefabs_atlas = atlases.add(TextureAtlas::from_grid(
		prefabs_texture,
		Vec2::new(16.0, 16.0),
		1,
		5,
	));
	commands.insert_resource(PrefabAtlas(prefabs_atlas.clone()));

	commands.insert_resource(MapMaterials {
		box_mat: box_material.clone(),
		box_mesh: box_mesh.clone(),
	});

	// let prefab_mesh = meshes.add(Mesh::from(shape::Plane { size: 0.75 }));

	let mut pillars = Vec::new();
	for x in 0..16 {
		for y in 0..16 {
			pillars.push(
				commands
					.spawn_bundle(PillarBundle {
						pillar: Pillar(x, y),
						mesh: PbrBundle {
							mesh: box_mesh.clone(),
							material: box_material.clone(),
							transform: Transform {
								translation: Vec3::new(x as f32, 0.0, y as f32) * BOX_SCALE,
								scale: Vec3::new(1.0, 1.0, 1.0) * BOX_SCALE,
								..Default::default()
							},
							..Default::default()
						},
						..Default::default()
					})
					.with_children(|parent| {
						parent
							.spawn_bundle(SpriteSheetBundle {
								// mesh: prefab_mesh.clone(),
								texture_atlas: prefabs_atlas.clone(),
								sprite: TextureAtlasSprite::new(0),
								transform: Transform {
									translation: Vec3::new(0.0, 8.1, 0.0),
									rotation: Quat::from_rotation_x(-90f32.to_radians())
										* Quat::from_rotation_z(-90f32.to_radians()),
									scale: Vec3::splat(1.0 / 16.0),
								},
								visible: Visible {
									is_transparent: true,
									is_visible: true,
								},
								..Default::default()
							})
							.insert(PrefabIcon);
					})
					.id(),
			);
		}
	}

	commands
		.spawn()
		.insert_bundle((
			Transform::from_translation(Vec3::new(-7.5, -8.0, -7.5)),
			GlobalTransform::default(),
		))
		.push_children(&pillars)
		.id();
}

#[derive(Default)]
pub struct Pillar(pub usize, pub usize);

#[derive(Bundle, Default)]
pub struct PillarBundle {
	pub pillar: Pillar,
	#[bundle]
	pub mesh: PbrBundle,
	pub selectable: RayCastMesh<SelectableRaycastSet>,
	pub selection: Selectable,
}

pub fn update_map_display(
	map: Res<MapResource>,
	mut query: Query<(&Pillar, &mut Transform)>,
) {
	if map.is_changed() {
		for (pillar, mut transform) in query.iter_mut() {
			let height = map.0.heights.0[pillar.1][pillar.0].0;
			transform.translation.x = pillar.0 as f32 * BOX_SCALE;
			transform.translation.z = pillar.1 as f32 * BOX_SCALE;
			transform.translation.y = height as f32 * BOX_SCALE;
		}
	}
}

pub fn update_prefabs(
	map: Res<MapResource>,
	mut query: Query<
		(&Parent, &mut TextureAtlasSprite, &mut Visible),
		With<PrefabIcon>,
	>,
	q_parent: Query<&Pillar>,
) {
	if map.is_changed() {
		for (parent, mut sprite, mut visible) in query.iter_mut() {
			if let Ok(pillar) = q_parent.get(parent.0) {
				let prefab = &map.0.prefabs.0[pillar.1][pillar.0];
				sprite.index = match prefab {
					Prefab::None => 4,
					Prefab::Melee => 4,
					Prefab::Projectile => 3,
					Prefab::JumpPad => 2,
					Prefab::Stairs => 1,
					Prefab::Hideous => 0,
				};
				visible.is_visible = prefab != &Prefab::None;
			}
		}
	}
}

struct PrefabAtlas(Handle<TextureAtlas>);

pub struct PrefabIcon;
