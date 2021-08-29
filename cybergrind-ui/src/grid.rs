use bevy::prelude::*;
use bevy_prototype_debug_lines::*;

use crate::map3d::BOX_SCALE;

pub fn draw_grid(mut lines: ResMut<DebugLines>) {
	const LINE_COUNT: f32 = 32.0;
	const COLOR: Color = Color::DARK_GRAY;
	let offset = Vec3::new(-(LINE_COUNT / 2.0), 0.0, -(LINE_COUNT / 2.0));
	for x in 0..=(LINE_COUNT as usize) {
		lines.line_colored(
			Vec3::new(x as f32 * BOX_SCALE, 0.0, 0.0) + offset,
			Vec3::new(x as f32 * BOX_SCALE, 0.0, LINE_COUNT * BOX_SCALE) + offset,
			0.0,
			COLOR,
		);
	}
	for y in 0..=(LINE_COUNT as usize) {
		lines.line_colored(
			Vec3::new(0.0, 0.0, y as f32 * BOX_SCALE) + offset,
			Vec3::new(LINE_COUNT * BOX_SCALE, 0.0, y as f32 * BOX_SCALE) + offset,
			0.0,
			COLOR,
		);
	}

	const ARROW_COLOR: Color = Color::RED;
	// draw arrow
	let o = Vec3::new(LINE_COUNT / 8.0 - 0.25, 0.25, LINE_COUNT / 2.0) + offset;
	let o_tip = o + Vec3::new(2.5, 0.0, 0.0);
	lines.line_colored(o, o_tip, 0.0, ARROW_COLOR);
	lines.line_colored(
		o_tip,
		o_tip + Vec3::new(-1.0, 0.0, -0.75),
		0.0,
		ARROW_COLOR,
	);
	lines.line_colored(
		o_tip,
		o_tip + Vec3::new(-1.0, 0.0, 0.75),
		0.0,
		ARROW_COLOR,
	);
}
