fn function() {
    println!("{}", "start function")
}

mod mod1 {
    pub fn function() {
        super::function();
    }

    pub mod mod2 {
        fn function() {
            println!("start function 2")
        }

        pub fn call() {
            self::function()
        }
    }
}

fn main() {
    mod1::mod2::call()
}
