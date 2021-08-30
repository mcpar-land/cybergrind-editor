use bevy::{prelude::*, render::texture::ImageType};
use bevy_mod_picking::PickableBundle;
use cybergrind_core::Map;

pub const BOX_SCALE: f32 = 1.0;

pub struct MapResource(pub cybergrind_core::Map);

pub fn spawn_map(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut textures: ResMut<Assets<Texture>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let box_mesh = meshes.add(Mesh::from(shape::Box::new(1.0, 16.0, 1.0)));
	let box_texture = Texture::from_buffer(
		include_bytes!("../assets/grid.png"),
		ImageType::Extension("png"),
	)
	.unwrap();
	let box_material = materials.add(textures.add(box_texture).into());

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
					})
					.insert_bundle(PickableBundle::default())
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

pub struct Pillar(pub usize, pub usize);

#[derive(Bundle)]
pub struct PillarBundle {
	pub pillar: Pillar,
	#[bundle]
	pub mesh: PbrBundle,
}

impl Pillar {
	pub fn update(&self, map: &Map, transform: &mut Transform) {
		let height = map.heights.0[self.1][self.0].0;
		transform.translation.x = self.0 as f32 * BOX_SCALE;
		transform.translation.z = self.1 as f32 * BOX_SCALE;
		transform.translation.y = height as f32 * BOX_SCALE;
	}
}

pub fn update_map_display(
	map: Res<MapResource>,
	mut query: Query<(&Pillar, &mut Transform)>,
) {
	for (pillar, mut transform) in query.iter_mut() {
		pillar.update(&(*map).0, &mut transform);
	}
}
