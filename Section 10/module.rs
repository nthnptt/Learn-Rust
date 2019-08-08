pub mod a {
    pub mod serie {
        pub mod of {
            pub fn nested_module() {}
        }
    }
}
use a::serie::of::nested_module;
fn main() {
    nested_module();
}
