fn main() {
    println!("string, literal!");
    // string literal

    let mut s = String::from("hello");
    s.push_str(", world!"); //push_str() appends a literal to a String

    println!("{}", s);

    println!("Memory and Allocation"); 

    println!("Variables and data interacting with clone");

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    takes_ownership(s);

    let x = 5;

    makes_copy(x);
}

fn takes_ownership(some_string:String){

    println!("{} ownership taken", some_string);

}

fn makes_copy(some_integer: i32){
    println!("{} make copy" , some_integer);
}