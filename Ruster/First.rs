use std::io;

fn main() {
   fn lambda(um: i128, dois: i128) -> i128 {
        um + dois
   }
   println!("{}", lambda(10, 5));
   fn verify_i(n1: i32, n2: i32) -> i32 {
        if n1 < n2 {n2} else {n1}
   }

    let mut inventory2: Vec<String> = Vec::new();
    loop {
        let mut pusher = String::new();
        io::stdin().read_line(&mut pusher).expect("Inválido");
    
        if pusher.trim() == "end" {
            break;
        }
    
    inventory2.push(pusher.trim().to_string());  
}
fn shipping(a: &str, b: &str) {
    if (a == "Kris" && b == "Noelle") || (a == "Noelle" && b == "Kris") {
        println!("Kriselle");
    } else {
        println!("Não shippa, newbie");
    }
}

shipping("Kris", "Noelle");






}
// Cortei parte do código pra ficar menor e reduzir o tempo de compilação
