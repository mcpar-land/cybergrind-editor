use bevy::{
	prelude::*,
	render::{mesh::Indices, pipeline::PrimitiveTopology},
};

pub fn pillar_mesh() -> Mesh {
	let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

	mesh.set_attribute(
		Mesh::ATTRIBUTE_POSITION,
		vec![
			[0.5, 8.0, 0.5],
			[0.5, 8.0, -0.5],
			[-0.5, 8.0, 0.5],
			[-0.5, 8.0, -0.5],
			[0.5, 7.0, 0.5],
			[0.5, 7.0, -0.5],
			[-0.5, 7.0, 0.5],
			[-0.5, 7.0, -0.5],
			[0.5, -8.0, 0.5],
			[0.5, -8.0, -0.5],
			[-0.5, -8.0, 0.5],
			[-0.5, -8.0, -0.5],
		],
	);

	mesh.set_indices(Some(Indices::U32(vec![
		0, 1, 2, //
		1, 2, 3, //
		1, 3, 5, //
		3, 5, 7, //
		2, 3, 7, //
		2, 6, 7, //
		0, 2, 6, //
		0, 4, 6, //
		0, 1, 5, //
		0, 4, 5, //
		5, 7, 9, //
		7, 9, 11, //
		6, 7, 11, //
		6, 10, 11, //
		4, 6, 8, //
		6, 8, 10, //
		4, 5, 9, //
		4, 8, 9, //
		8, 9, 10, //
		9, 10, 11,
	])));

	todo!();
}
