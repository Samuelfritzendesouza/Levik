fn main() {
    println!("I'm not gonna sugarcoat it!");
 
    let algo_nah: &str = "String";
    println!("{}", algo_nah);
    let one: &str = "Anderson";
    let two: &str = "14";
    let three: &str = "1.72";
    println!("{one}, {two}, {three}");
    let a: i32 = 10;
    let b: i32 = 3;
    println!("{}", a + b);
    println!("{}", a - b);
    println!("{}", a * b);
    println!("{}", a / b);
    println!("{}", a % b);
    let c: i16 = 5;
    if c % 2 != 0 {
        println!("Impar, eu acho");
    }
    for n in  i32 10 {
        println!("{}", n);
    }
}