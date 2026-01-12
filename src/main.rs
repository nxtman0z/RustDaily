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



fn main(){
    'outer:loop{
        println!("entered the outer loop");
        
        'inner:loop{
            println!("entered the inner loop");

            break 'outer;
        }
        println!("this point will never be reached");
    }
    println!("exited from the outer loop");
}