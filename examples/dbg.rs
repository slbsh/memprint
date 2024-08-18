use memprint::MemPrint;

#[derive(MemPrint)]
struct MyStruct {
    c: u8,
    a: i32,
    b: f32,
}

impl MyStruct {
    fn new(a: i32, b: f32, c: u8) -> Self {
        Self { a, b, c }
    }
}


fn main() {
    let mut vec = Vec::new();

    vec.push(MyStruct::new(42,    3.14, b'A'));
    vec.push(MyStruct::new(69,    2.71, b'B'));
    vec.push(MyStruct::new(1337,  1.41, b'C'));
    vec.push(MyStruct::new(9001,  0.0,  b'D'));
    vec.push(MyStruct::new(80085, -1.0, b'E'));

    MyStruct::memprint_block(&vec);
}
