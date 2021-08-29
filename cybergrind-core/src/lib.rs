use nom::{
	branch::alt,
	character::complete::{char, digit1, line_ending, one_of},
	combinator::{map, opt, recognize},
	multi::{many1, separated_list1},
	sequence::{delimited, pair, separated_pair},
};

pub trait Parsable: Sized + ToString {
	fn parse(input: &str) -> nom::IResult<&str, Self>;

	fn default() -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Map {
	pub heights: Grid<Height>,
	pub prefabs: Grid<Prefab>,
}

impl Map {
	pub fn from_str(
		input: &str,
	) -> Result<Self, nom::Err<nom::error::Error<&str>>> {
		Self::parse(input).map(|v| v.1)
	}
}

impl Parsable for Map {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		map(
			separated_pair(
				Grid::<Height>::parse,
				pair(line_ending, line_ending),
				Grid::<Prefab>::parse,
			),
			|(heights, prefabs)| Self { heights, prefabs },
		)(input)
	}

	fn default() -> Self {
		Self {
			heights: Grid::<Height>::default(),
			prefabs: Grid::<Prefab>::default(),
		}
	}
}

impl ToString for Map {
	fn to_string(&self) -> String {
		format!(
			"{}\n\n{}",
			self.heights.to_string(),
			self.prefabs.to_string()
		)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Grid<T: Parsable>(pub [[T; 16]; 16]);

impl<T: Parsable + Copy> Parsable for Grid<T> {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		fn to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
			use std::convert::TryInto;

			v.try_into().unwrap_or_else(|v: Vec<T>| {
				panic!("Expected a Vec of length {} but got length {}", N, v.len())
			})
		}

		map(separated_list1(line_ending, many1(T::parse)), |vals| {
			Self(to_array(vals.into_iter().map(|v| to_array(v)).collect()))
		})(input)
	}

	fn default() -> Self {
		Self([[T::default(); 16]; 16])
	}
}

impl<T: Parsable> ToString for Grid<T> {
	fn to_string(&self) -> String {
		let mut s = String::new();
		for (i, row) in self.0.iter().enumerate() {
			for item in row {
				s.push_str(&item.to_string());
			}
			if i < self.0.len() - 1 {
				s.push('\n');
			}
		}
		s
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Height(pub i8);

impl Parsable for Height {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		let single = recognize(one_of("0123456789"));
		let paren = recognize(pair(opt(char('-')), digit1));
		map(
			alt((single, delimited(char('('), paren, char(')')))),
			|s: &str| Self(s.parse::<i8>().expect("Could not parse height")),
		)(input)
	}

	fn default() -> Self {
		Self(0)
	}
}

impl ToString for Height {
	fn to_string(&self) -> String {
		if self.0 <= 9 && self.0 >= 0 {
			format!("{}", self.0)
		} else {
			format!("({})", self.0)
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Prefab {
	None,
	Melee,
	Projectile,
	JumpPad,
	Stairs,
	Hideous,
}

impl Parsable for Prefab {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		map(one_of("0npJsH"), |c| match c {
			'0' => Prefab::None,
			'n' => Prefab::Melee,
			'p' => Prefab::Projectile,
			'J' => Prefab::JumpPad,
			's' => Prefab::Stairs,
			'H' => Prefab::Hideous,
			_ => panic!("Invalid prefab character"),
		})(input)
	}

	fn default() -> Self {
		Self::None
	}
}

impl ToString for Prefab {
	fn to_string(&self) -> String {
		match self {
			Prefab::None => "0",
			Prefab::Melee => "n",
			Prefab::Projectile => "p",
			Prefab::JumpPad => "J",
			Prefab::Stairs => "s",
			Prefab::Hideous => "H",
		}
		.to_string()
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn parse_height() {
		assert_eq!(Height::parse("(15)").unwrap().1, Height(15));
		assert_eq!(Height::parse("(-2)").unwrap().1, Height(-2));
		assert_eq!(Height::parse("0").unwrap().1, Height(0));
		assert_eq!(Height::parse("8").unwrap().1, Height(8));
		assert_eq!(Height::parse("55").unwrap().1, Height(5));
	}

	#[test]
	fn parse_height_list() {
		let r = many1(Height::parse)("(-8)22211000(-1)(-2)(-2)(-2)(-1)00").unwrap();
		println!("{:?}", r);
	}

	#[test]
	fn parse_prefab() {
		assert_eq!(Prefab::parse("0").unwrap().1, Prefab::None);
		assert_eq!(Prefab::parse("n").unwrap().1, Prefab::Melee);
		assert_eq!(Prefab::parse("p").unwrap().1, Prefab::Projectile);
		assert_eq!(Prefab::parse("J").unwrap().1, Prefab::JumpPad);
		assert_eq!(Prefab::parse("s").unwrap().1, Prefab::Stairs);
		assert_eq!(Prefab::parse("H").unwrap().1, Prefab::Hideous);
	}

	const TEST_MAP: &'static str = r#"222211000(-1)(-2)(-2)(-2)(-1)00
2222(-15)(-15)(-15)00(-15)(-15)(-15)(-15)(-15)0(-15)
(-15)(-15)(-15)(-15)(-15)(-15)(-15)00(-15)(-15)(-15)(-15)(-15)0(-15)
(-15)(-15)(-15)(-15)(-15)(-15)(-15)00(-15)(-15)(-15)(-15)(-15)0(-15)
222221000012100(-15)
222221000012100(-15)
(-15)(-15)22(-15)(-15)(-15)00(-15)(-15)(-15)(-15)(-15)0(-15)
(-15)(-15)11(-15)(-15)(-15)00(-15)(-15)(-15)(-15)(-15)0(-15)
(-15)(-15)00(-15)(-15)(-15)00(-15)(-15)00000
(-15)(-15)0001000(-15)(-15)00000
(-15)(-15)0001000(-15)(-15)00111
(-15)(-15)(-15)(-15)(-15)(-15)(-15)00(-15)(-15)00111
(-15)(-15)(-15)(-15)(-15)(-15)(-15)00(-15)(-15)00111
555(-15)(-15)(-15)(-15)00(-15)(-15)00111
555321000(-15)(-15)00111
555(-15)(-15)(-15)(-15)000000111

ppnnsnsnnssnsspp
ppnnJJJnnJJJJJpJ
JJJJJJJnnJJJJJnJ
JJJJJJJnnJJJJJnJ
ppnnnssppssnsspJ
ppnnnssppssnsspJ
JJnnJJJnnJJJJJnJ
JJssJJJnnJJJJJnJ
JJssJJJnnJJ00000
JJppsnsnnJJ00sss
JJppsnsnnJJ0sppp
JJJJJJJnnJJ0sppp
JJJJJJJnnJJ0s000
pppJJJJnnJJ0s000
pppssssppJJ0s0H0
pppJJJJppnn0s000"#;

	#[test]
	fn parse_in() {
		let parsed = Map::parse(TEST_MAP).unwrap().1;

		println!("{:#?}", parsed);
	}

	#[test]
	fn parse_out() {
		let parsed = Map::parse(TEST_MAP).unwrap().1;
		let serialized = parsed.to_string();
		assert_eq!(serialized, TEST_MAP.to_string());
	}
}
