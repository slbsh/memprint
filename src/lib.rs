#[doc = include_str!("../README.md")]

pub use memprint_derive::MemPrint;

use colored::{Colorize, Color};

pub struct Field {
	name: &'static str,
	span: (usize, usize),
	type_name: &'static str,
}

fn print_slice_bytes(bytes: &[u8], color: Option<Color>) {
	bytes.iter().for_each(|byte| print!("{}", format!("{:02x}", byte)
		.color(color.unwrap_or(Color::White))));
}

impl Field {
	pub fn new<T: ?Sized>(name: &'static str, span: (usize, usize), _: &T) -> Self {
		Self {
			name, span,
			type_name: std::any::type_name::<T>(),
		}
	}
}

pub enum DataType {
	Struct,
	TupleStruct,
	UnitStruct,
	Enum,
	Union,
	Primitive,
}

pub trait MemPrint {
	fn data_type(&self) -> DataType;
	fn fields(&self) -> Option<Vec<Field>> { None }


	fn memprint(obj: &Self) where Self: Sized {
		unsafe {
			Self::memprint_raw(obj as *const Self);
		}
	}

	/// # Safety
	/// ptr must satisfy all the invariants of std::ptr::NonNull
	unsafe fn memprint_raw(ptr: *const Self) where Self: Sized {
		use std::cmp;

		let ptr = std::ptr::NonNull::new_unchecked(ptr as *mut Self);

		let bytes = std::slice::from_raw_parts(
			ptr.as_ptr() as *const u8, 
			std::mem::size_of::<Self>()
		);

		match ptr.as_ref().data_type() {
			DataType::Struct => {
				let fields = &ptr.as_ref().fields()
					.expect("For structs with no fields make this empty");

				if fields.is_empty() {
					println!("{} {{ }}", std::any::type_name::<Self>());
					return;
				}

				println!("{} {{", std::any::type_name::<Self>());

				let (mw_name, mw_type) = fields.iter().fold((0, 0), |acc, f|
					(cmp::max(acc.0, f.name.len()), cmp::max(acc.1, f.type_name.len())));

				fields.iter().for_each(|field| {
					print!("    {}: {}", field.name,  " ".repeat(mw_name - field.name.len()));
					print!("{}  {}", field.type_name, " ".repeat(mw_type - field.type_name.len()));

					print_slice_bytes(&bytes[field.span.0..field.span.1], None);
					println!();
				});

				println!("}}");
			},
			DataType::TupleStruct => {
				let fields = ptr.as_ref().fields()
					.expect("For structs with no fields make this empty");

				if fields.is_empty() {
					println!("{}()", std::any::type_name::<Self>());
					return;
				}

				println!("{}(", std::any::type_name::<Self>());

				let (mw_name, mw_type) = fields.iter().fold((0, 0), |acc, f|
					(cmp::max(acc.0, f.name.len()), cmp::max(acc.1, f.type_name.len())));

				fields.iter().for_each(|field| {
					print!("    {}", " ".repeat(mw_name - field.name.len()));
					print!("{}  {}", field.type_name, " ".repeat(mw_type - field.type_name.len()));

					print_slice_bytes(&bytes[field.span.0..field.span.1], None);
					println!();
				});

				println!(")");
			},
			DataType::UnitStruct => println!("{}", std::any::type_name::<Self>()),
			DataType::Primitive => {
				print_slice_bytes(bytes, None);
				println!();
			},
			_ => todo!("implement enums and unions"),
		}

	}

	fn memprint_simple(obj: &Self) where Self: Sized {
		unsafe {
			Self::memprint_simple_raw(obj as *const Self, obj.fields().as_deref());
		}
	}

	unsafe fn memprint_simple_raw(ptr: *const Self, fields: Option<&[Field]>) where Self: Sized {
		const COLORS: [Color; 8] = [
			Color::Red,
			Color::Green,
			Color::Yellow,
			Color::Blue,
			Color::Magenta,
			Color::Cyan,
			Color::White,
			Color::Black,
		];

		let ptr = std::ptr::NonNull::new_unchecked(ptr as *mut Self);

		match ptr.as_ref().data_type() {
			DataType::Struct | DataType::TupleStruct => {
				fields.expect("For structs with no fields make this empty")
					.iter().enumerate().for_each(|(i, field)| {
					let color = COLORS.get_unchecked(i % COLORS.len());

					let bytes = std::slice::from_raw_parts(
						ptr.as_ptr() as *const u8, 
						std::mem::size_of::<Self>()
					);

					print_slice_bytes(&bytes[field.span.0..field.span.1], Some(*color));
				})
			},
			DataType::UnitStruct => {
				println!("_");
			},
			DataType::Primitive => {
				let bytes = std::slice::from_raw_parts(
					ptr.as_ptr() as *const u8, 
					std::mem::size_of::<Self>()
				);

				print_slice_bytes(bytes, None);
			},
			_ => todo!("implement enums and unions"),
		}

	}

	fn memprint_block(arr: &[Self]) where Self: Sized {
		unsafe {
			Self::memprint_block_raw(arr.as_ptr(), arr.len());
		}
	}

	/// # Safety
	/// ptr must satisfy all the invariants of std::ptr::NonNull
	unsafe fn memprint_block_raw(ptr: *const Self, len: usize) where Self: Sized {
		let ptr = std::ptr::NonNull::new_unchecked(ptr as *mut Self);
		let fields = ptr.as_ref().fields();
		if let Some(fields) = &fields {
			// heading
			let mut pads = Vec::with_capacity(fields.len());

			fields.iter().for_each(|f| 
				pads.push(((f.span.1 - f.span.0) << 1).saturating_sub(f.name.len())));

			fields.iter().enumerate().for_each(|(i, field)|
				print!("{} {}", field.name, " ".repeat(pads[i].saturating_sub(1))));

			println!();
		};

		// block
		(0..len).for_each(|i| {
			Self::memprint_simple_raw(ptr.as_ptr().add(i), fields.as_deref());
			println!();
		});
	}
}

impl MemPrint for () {
	fn data_type(&self) -> DataType { DataType::UnitStruct }
}

macro_rules! impl_primitive {
	($($t:ty),+) => {
		$(
			impl MemPrint for $t {
				fn data_type(&self) -> DataType { DataType::Primitive }
			}
		)+
	}
}

impl_primitive! {
	u8, u16, u32, u64, u128,
	i8, i16, i32, i64, i128,
	f32, f64,
	bool,
	char
}
