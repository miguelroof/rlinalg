pub mod vector;


use rand::Rng;
use std::cmp::Ordering;
use std::io;

static PRESITION:f64 = 1e-9;

fn main2() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");
    loop {
        println!("Please insert your guessed number");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("YOu guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You Win!!!");
                break;
            }
        }
    }
}

fn main() {
    // addition
    let chk: bool = false;
    if chk {
        let sum = 5 + 10;

        // subtraction
        let difference = 95.5 - 4.3;

        // multiplication
        let product = 4 * 30;

        // division
        let quotient = 56.7 / 32.2;
        let truncated = -5 / 3; // Results in -1
        let remainder = 43 % 5; // resto
        let mut module: i32 = -5;
        module = module.rem_euclid(3);

        println!("My values are suma={sum}, resta={difference}, producto={product}, division={quotient}, truncamiento={truncated}, resto={remainder}, módulo={module}");
        // remainder

        // las tuplas no se pueden modificar, pero podemos tener distintos tipos de elementos. tamano fijo
        let tup = (500, 6.4, 1);
        // let (x,y,z) = tup;
        let _aa = tup.1;
        // tup.1 = 3.2; Para modificar una tupla hay que hacerla mutable

        //array
        let array = [1, 2, 3, 4, 5, 6]; // los arrays tienen que ser todos del mismo tipo
        let _aa = array[0];
        let array: [u32; 6] = [3; 6];
        println!("array {:?}", array);

        //boolean
        let _t: bool = true;

        //char
        let _z: char = 'z'; // los caracteres usan comilla simple

        //
        if false {
            main2();
        } else if false {
            println!("otra prueba");
        } else {
            println!("he tomado la última opción")
        }
        let a: i32 = another_function(sum);
        println!("Valor devuelto de {sum} es {a}");

        // el punto y coma diferencia si hay devolucion o no


        //........................loops................................................................
        let number_condition = if a < 10 { true } else { false };
        println!("{number_condition}");
        let mut counter: u32 = 0;
        let result_loop: u32 = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };
        println!("result_loop {result_loop}");

        // puedo especificar nombre para los loops
        let mut count = 0;
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;
            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break; // esto me saca del bucle inmediato
                }
                if count == 2 {
                    break 'counting_up; // me saca del superbucle
                }
                remaining -= 1;
            }
            count += 1;
        }

        // ................................while........................................................
        count = 0;
        while count != 9 {
            count += 1;
        }
        println!("End count = {count}");

        //.................................for..........................................................
        for element in array {
            println!("Elemento {element}");
        }
        for element in 0..4 {
            println!("Cuenta a tras {element}");
        }

        let mut s = String::from("hello");
        change_string(&mut s);
        println!("{s}");
        let _first_word_len = first_word_as_bytes(&s);
        let word = first_word_as_slice(&s);
        println!("{}", word);
    }

    structs_examples();
    vector_example();

}


fn another_function(x: i32) -> i32 {
    x * 3 // no pongo dobles comillas para que devuelva un valor
}

// fn function_that_return_tuple (x: i32) -> (i32, i32) {
//     (x*2, x*3)
// }

fn change_string(some_string: &mut String) {
    some_string.push_str(", word in mutable pointer");
}

fn first_word_as_bytes(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len() // esto es lo que devuelvo si no encuentro nada
}

fn first_word_as_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// ....................................STRUCTS.....................................................

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email,
        username,
        sign_in_count: 1,
    }
}


fn structs_examples() {
    // funcion para contener todos los tipos de estructuras
    println!(".................Ejemplos de struct.................");
    let _user1 = User {
        active: true,
        username: String::from("minombreusuario"),
        email: String::from("mtejada@gmail.com"),
        sign_in_count: 1,
    };
    let _user1 = build_user(String::from("miotronombre"), String::from("tejada.miguel@gmail.com"));
    let _user2 = User {
        email: String::from("cuchi@hotmail.com"),
        .._user1
    };
    // como copio datos de username de _user1, este dato ya no podra ser usado
    // _user1.email = String::from("pepito@ppp.com"); -> ESTO PRODUCE UN FALLO.
    // println!("{:?}", _user1); // no puedo porque he prestado uno de los valores
    println!("{:?}", _user2);
    let ret = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{:?}, {}", ret, ret.area());
}

//........................................ALG_VECT
fn vector_example() {
    let mut v = vector::Vector(1.0,2.0,2.0);
    println!("{:?}", v);
    v.0 = 3.3;
    println!("{:?}", PRESITION);


}