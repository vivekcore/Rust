fn main() {
    let a: i32 = -100;
    let b: f32 = 16.393;
    let c: u8 = 2;

    println!("a: {}, b: {}, c: {}", a, b, c);

    let is_male: bool = true;
    let is_above_18: bool = true;

    if is_male {
        println!("You are a male");
    } else {
        println!("You are not a male");
    }

    if is_male && is_above_18 {
        println!("You are a legal male");
    }

    //string

    let greetiing: String = String::from("helllo world ");
    println!("{}", greetiing);

    //loop

    for i in 0..10 {
        print!("{} ", i);
    }

    //function

    let sentence = String::from("my name is vivek");
    let first_word = get_first_word(sentence);

    println!("\n{} ", first_word);

    //mutatibility
    //every varible is immutable by default
    let mut a = String::from("Hi");
    a.push_str("abc");

    println!(
        "Capacity: {}, Length: {}, pointer: {:p}",
        a.capacity(),
        a.len(),
        a.as_ptr()
    );

    //owenership

    let b = a;
    println!("{}", b);
    // println!("{}",a); error

    let mut my_string = String::from("hello ");
    my_string = take_ownership(my_string); 

     println!("{}",my_string);
    
}

fn take_ownership(some_string: String) -> String {
    println!("{}", some_string);
    return  some_string;
}

fn get_first_word(sentence: String) -> String {
    let mut ans = String::new();
    for char in sentence.chars() {
        if char == ' ' {
            break;
        }
        ans.push(char);
    }

    ans
}
