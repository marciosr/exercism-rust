extern crate int_enum;
extern crate enum_iterator;

use int_enum::IntEnum;
use enum_iterator::IntoEnumIterator;

#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, IntEnum, IntoEnumIterator, PartialOrd, Eq, Ord)]
pub enum ResistorColor {
	Black = 0,
	Blue = 6,
	Brown = 1,
	Green = 5,
	Grey = 8,
	Orange = 3,
	Red = 2,
	Violet = 7,
	White = 9,
	Yellow = 4
}

pub fn color_to_value(color: ResistorColor) -> usize {
	color as usize
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value) {
        Ok(resistor) => format!("{:?}", resistor),
        Err(_) => "value out of range".to_string()
    }
}

pub fn colors() -> Vec<ResistorColor> {
    ResistorColor::into_enum_iter().collect()
}

#[cfg(test)]
mod tests {
	use super::*;
	//use resistor_color::{color_to_value, colors, value_to_color_string, ResistorColor};

	#[test]
	fn test_black() {
		assert_eq!(color_to_value(ResistorColor::Black), 0);
	}

	#[test]
	fn test_orange() {
		assert_eq!(color_to_value(ResistorColor::Orange), 3);
	}

	#[test]
	fn test_white() {
		assert_eq!(color_to_value(ResistorColor::White), 9);
	}

	#[test]
	fn test_2() {
		assert_eq!(value_to_color_string(2), String::from("Red"));
	}

	#[test]
	fn test_6() {
		assert_eq!(value_to_color_string(6), String::from("Blue"));
	}

	#[test]
	fn test_8() {
		assert_eq!(value_to_color_string(8), String::from("Grey"));
	}

	#[test]
	fn test_11_out_of_range() {
		assert_eq!(
		    value_to_color_string(11),
		    String::from("value out of range")
		);
	}

	#[test]
	fn test_all_colors() {
		use ResistorColor::*;
		println!("All colors: {:?}", colors());
		assert_eq!(
		    colors(),
		    vec![Black, Brown, Red, Orange, Yellow, Green, Blue, Violet, Grey, White]
		);
	}
}
