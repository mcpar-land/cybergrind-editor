use std::ops::RangeInclusive;

pub fn is_inside_box(
	(x, y): (usize, usize),
	((x1, y1), (x2, y2)): &((usize, usize), (usize, usize)),
) -> bool {
	let x_range = if x1 >= x2 { x2..=x1 } else { x1..=x2 };
	let y_range = if y1 >= y2 { y2..=y1 } else { y1..=y2 };
	x_range.contains(&&x) && y_range.contains(&&y)
}
