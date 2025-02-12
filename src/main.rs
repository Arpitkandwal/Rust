
use std::io;
mod calculator;

fn main() {
    println!("Enter the first number:");
    let num1 = get_input();

    println!("Enter the second number:");
    let num2 = get_input();

    let add = calculator::Operation::Add(num1, num2);
    let subtract = calculator::Operation::Subtract(num1, num2);
    let multiply = calculator::Operation::Multiply(num1, num2);
    let division = calculator::Operation::Division(num1, num2);

    println!("Results:");
    println!("Addition: {}", calculator::calculate(add));
    println!("Subtraction: {}", calculator::calculate(subtract));
    println!("Multiplication: {}", calculator::calculate(multiply));

    if num2 != 0.0 {
        println!("Division: {}", calculator::calculate(division));
    } else {
        println!("Division: Cannot divide by zero");
    }


    let s1 = String::from("Hello");
    let s2 = String::from("World");

    // let concatenated_string = concatenate_string(&s1, &s2);
    // println!("{}", concatenated_string);

    let  book = Book {
        title: String::from("Way of Life"),
        author: String::from("Arpit"),
        publication_year: 2025,
    };

    let book_data = get_book(book);

    for data in book_data {
        println!("{}", data);
    }
}


fn get_input() -> f64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
        .trim()
        .parse()
        .expect("Please enter a valid number")
}

struct Book {
    title: String,
    author: String,
    publication_year: u32,
}

fn get_book(book: Book) -> [String; 3] {
    let title = book.title;
    let author = book.author;
    let publication_year = book.publication_year;

    let data: [String; 3] = [title, author, publication_year.to_string()];

    return data
}




fn concatenate_string(s1: &str, s2: &str) -> String {
    let mut result = String::new();

    result.push_str(s1);
    result.push_str(" ");
    result.push_str(s2);

    return result;
}


