use rand::Rng;

#[no_mangle]
pub fn sum (a: i32, b: i32) -> i32 { 
   println!("inside callee");
let mut rng = rand::thread_rng();
let num = rng.gen::<i32>();
println!("inside callee, random path: {}", num);
a + b
}