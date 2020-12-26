use fnumbuf::FnumBuf;

#[derive(Debug)]
struct ShowDrop;

impl Drop for ShowDrop {
    fn drop(&mut self) {
        println!("drop!");
    }
}

#[derive(Debug, fnum::Fnum)]
enum MyEnum {
    A(ShowDrop),
    B
}

fn main() {
    {
        println!("1");
        let mut buf = FnumBuf::new();
        buf.push(MyEnum::A(ShowDrop));
        buf.push(MyEnum::B);
        buf.push(MyEnum::A(ShowDrop));
    }
    {
        println!("2");
        let mut buf = FnumBuf::new();
        buf.push(MyEnum::A(ShowDrop));
        buf.push(MyEnum::B);
        buf.push(MyEnum::A(ShowDrop));
        buf.iter().for_each(|_| ());
    }
    {
        println!("3");
        let mut buf = FnumBuf::new();
        buf.push(MyEnum::A(ShowDrop));
        buf.push(MyEnum::B);
        buf.push(MyEnum::A(ShowDrop));
        buf.into_iter().for_each(|_| ());
    }
}
