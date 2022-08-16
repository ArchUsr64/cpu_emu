extern crate json;
use json::JsonValue;

pub trait ToNum {
	fn to_num(&self) -> u64;
}

impl ToNum for JsonValue{
	fn to_num(&self) -> u64 {
		match self {
			JsonValue::Number(val) => val.as_fixed_point_u64(0).unwrap(),  
			_ => 0u64,
		}
	}
}
