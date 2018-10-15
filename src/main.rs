extern crate ash_nazg;

fn main() {
    println!("Hello, world! {}", 5);
    ash_nazg::compose();

    println!(">>> ---------------------------------------------- <<<");
    ash_nazg::go();
}
