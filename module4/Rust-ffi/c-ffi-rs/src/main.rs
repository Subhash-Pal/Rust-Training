//#include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use c_binding_video::add_one;
fn main() {
    //println!("The value of OUT_DIR is: {}", env!("OUT_DIR"));
    unsafe{
    println!("{:?}" , add_one(1));
    }
}
