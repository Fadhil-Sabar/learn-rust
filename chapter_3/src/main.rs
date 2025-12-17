fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let arr = ["test"];

    println!("{:#?}", arr);

    let tuple = (1, 2.5, 3);

    println!("{}", tuple.1);

    let a = [3; 5];
    println!("{:#?}", a);

    //statement or expressions
    let y = {
        let x = 3;
        x + 1
    };

    // !NOTE: statement itu ga return value, expression return value
    // !NOTE: apapun yang ada di {} itu expression
    let x = five();

    println!("{x} - {y}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // !NOTE: loop bisa dikasih nama buat misahin break loop nya

    // fibonacci(2);
}

fn five() -> i32 {
    5
}

// fn fibonacci(n: i32) {
//     let mut result = String::new();
//     for each_n in 0..n + 1 {
//         println!("{each_n}");

//         let prev_n = {
//             each_n -
//         }

//         result.push_str(&(each_n).to_string());
//     }

//     println!("{result}")
// }
