use fnumbuf::FnumBuf;

#[derive(Debug, fnum::Fnum)]
enum MyEnum {
    A(u64),
    B(String),
    C(u64, u32, u32, u32),
    D {
        hello: u32,
        world: String,
    },
    E
}

fn main() {
    let mut buf = FnumBuf::new();
    buf.push(MyEnum::A(123));
    buf.push(MyEnum::A(45));
    buf.push(MyEnum::B(format!("hello")));
    buf.push(MyEnum::E);
    buf.push(MyEnum::A(123));
    buf.push(MyEnum::A(45));
    buf.push(MyEnum::B(format!("world")));
    println!("{:?}", &buf);

    for e in buf.iter() {
        println!("{:?}", e);
    }
    println!("===");
    for e in buf.into_iter() {
        println!("{:?}", e);
    }
}
