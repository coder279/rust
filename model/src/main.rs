mod mod1 {
    pub const Message: &str = "Hello World";
    pub(self) const NUMBER: u32 = 42;

    #[derive(Debug)]
    pub(crate) enum CrateEnum {
        Item = 4
    }

    pub mod mod2 {
        pub const Message: &str = "Hello World";

        pub fn say42() {
            println!("{}", super::NUMBER);
        }
    }
}

fn main() {
    println!("{:?}", mod1::mod2::Message);
    println!("{:#?}", mod1::CrateEnum::Item as u32); // 取值
}
