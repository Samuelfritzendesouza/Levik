use std::io;

fn main() {
   fn lambda(um: i128, dois: i128) -> i128 {
        um + dois
   }
   println!("{}", lambda(10, 999999999999999999999999999));
   fn verify_i(n1: i32, n2: i32) -> i32 {
        if n1 < n2 {n2} else {n1}
   }
   println!("{}", verify_i(55, 50));
    let mut num: i8 = 10;
    while num > 0 {
        println!("{num}");
        num -= 1 
    }
    println!("Num sei");
    let dia: i8= 7;
match dia {
    1 => println!("Segunda"),
    2 => println!("Terça"),
    3 => println!("Quarta"),
    4 => println!("Quinta"),
    5 => println!("Sexta"),
    6 => println!("Sábado"),
    7 => println!("Domingo"),
    _ => println!("Dia inválido"),
}
for n in 1..=20 {
    if n % 15 == 0 {
        println!("FizzBuzz");
    } else if n % 3 == 0 {
        println!("Fizz");
    } else if n % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{n}");
}
}
    let mut inventory:  Vec<&str> = Vec::new();
    inventory.push(&("Sword"));
    inventory.push(&("Poção"));
    inventory.push(&("Dinheiro"));

    println!("Inventário {:?}", inventory);
    println!("Tamanho: {}", inventory.len());

    let mut inventory2: Vec<String> = Vec::new();
    loop {
        let mut pusher = String::new();
        io::stdin().read_line(&mut pusher).expect("Inválido");
    
        if pusher.trim() == "end" {
            break;
        }
    
    inventory2.push(pusher.trim().to_string());  
}
}
// Cortei parte do código pra ficar menor e reduzir o tempo de compilação
