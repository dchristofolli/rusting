use std::io;

fn main(){
let mut vector = Vec::new();
vector.push(0);
println!("Enter the lenght of Fibonacci sequence you want");
let mut seq_lenght = String::new();
        io::stdin()
            .read_line(&mut seq_lenght)
            .expect("Failed to read line");
let seq_lenght = seq_lenght.parse::<usize>().unwrap();
while vector.len() <= seq_lenght{
    for element in vector.iter() {
        if element == 0{
            vector.push(1);
        }
    }
}

for n in vector.iter() {
    println!("{}", n);
    }
}