// fn main(){
//     println!("Hello, world!");
//     for i in 0..10{
//         println!("Number: {}", i);
//     }
// }

// fn main(){
//     let mut count =0;
//     println!("lets count untill infinity");
//     //infinity loop
//     loop{
//         count += 1;
//         if count == 3{
//             println!("three");
//             continue;
//         }
//         println!("{}", count);
//         if count == 5{
//             println!("ok thats enough");
//             break;
//         }


        
//     }
// }


// fn main(){
//     let mut counter = 0;
//     let result = loop{
//         counter += 1;

//         if counter == 10{
//             break counter *2;
//         }
//     };
//     assert_eq!(result, 20);
// }


//while loop
// fn main() {
//     let mut i = 0;
//     while i < 10 {
//         println!("hello");
//         i += 1;
//     }
// }



// fn main(){
//     for i in 1..6{
//         println!("{}",i);
//     }
// }


//for loop over arrays

// fn main(){
//     let arr =[10,20,30];
//     for x in arr{
//         println!("{}",x);
//     }
// }


//function


fn main() {
    println!("starts first");

    print_hello();

    let x = 10;
    let result = square(x);

    println!("square = {}", result);
}

fn print_hello() {
    println!("hello");
}

fn square(x: i32) -> i32 {
    x * x
}
