#[doc = include_str!("../README.md")]

pub use memprint_derive::MemPrint;

use colored::{Colorize, Color};

pub struct Field {
    name: &'static str,
    span: (usize, usize),
    type_name: &'static str,
}

fn print_slice_bytes(bytes: &[u8], color: Option<Color>) {
    bytes.iter().for_each(|byte| print!("{}", format!("{:02x}", byte).color(color.unwrap_or(Color::White))));
}

impl Field {
    pub fn new<T: ?Sized>(name: &'static str, span: (usize, usize), _: &T) -> Self {
        Self {
            name, span,
            type_name: std::any::type_name::<T>(),
        }
    }
}

pub trait MemPrint {
    fn get_fields(&self) -> Vec<Field>;

    fn memprint(obj: &Self) where Self: Sized {
        unsafe {
            Self::memprint_raw(obj as *const Self);
        }
    }

    unsafe fn memprint_raw(ptr: *const Self) where Self: Sized {
        use std::cmp;

        let Some(ptr) = std::ptr::NonNull::new(ptr as *mut Self) else {
            println!("NULLPTR");
            return;
        };

        let bytes = std::slice::from_raw_parts(
            ptr.as_ptr() as *const u8, 
            std::mem::size_of::<Self>()
        );

        println!("{} {{", std::any::type_name::<Self>());

        let fields = &ptr.as_ref().get_fields();

        let (mw_name, mw_type) = fields.iter().fold((0, 0), |acc, f| {
            (cmp::max(acc.0, f.name.len()), cmp::max(acc.1, f.type_name.len()))
        });

        for field in fields {
            let pad_name  = mw_name  - field.name.len();
            let pad_type  = mw_type  - field.type_name.len();

            print!("    {}: {}", field.name,  " ".repeat(pad_name));
            print!("{}  {}", field.type_name, " ".repeat(pad_type));

            print_slice_bytes(&bytes[field.span.0..field.span.1], None);
            println!();
        }

        println!("}}");
    }

    fn memprint_simple(obj: &Self) where Self: Sized {
        unsafe {
            Self::memprint_simple_raw(obj as *const Self, &obj.get_fields());
        }
    }

    unsafe fn memprint_simple_raw(ptr: *const Self, fields: &[Field]) where Self: Sized {
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

        for (i, field) in fields.iter().enumerate() {
            let color = COLORS.get_unchecked(i % COLORS.len());

            let bytes = std::slice::from_raw_parts(
                ptr as *const u8, 
                std::mem::size_of::<Self>()
            );

            print_slice_bytes(&bytes[field.span.0..field.span.1], Some(*color));
        }
    }

    fn memprint_block(arr: &[Self]) where Self: Sized {
        unsafe {
            Self::memprint_block_raw(arr.as_ptr(), arr.len());
        }
    }

    unsafe fn memprint_block_raw(ptr: *const Self, len: usize) where Self: Sized {
        let Some(ptr) = std::ptr::NonNull::new(ptr as *mut Self) else {
            println!("NULLPTR");
            return;
        };

        let fields = &ptr.as_ref().get_fields();

        // heading
        let mut pads = Vec::with_capacity(fields.len());

        fields.iter().for_each(|f| 
            pads.push(((f.span.1 - f.span.0) << 1).saturating_sub(f.name.len())));

        for (i, field) in fields.iter().enumerate() {
            print!("{} {}", field.name, " ".repeat(pads[i].saturating_sub(1)));
        }
        println!();


        // block
        for i in 0..len {
            Self::memprint_simple_raw(ptr.as_ptr().add(i), &fields);
            println!();
        }
    }
}
