pub mod k8s {
    #[path = ""]
    pub mod runtime {
        #[path = "runtime.v1.rs"]
        pub mod v1;
    }
}

fn main() {
    println!("Hello, world");
}
