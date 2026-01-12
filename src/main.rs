// fn main(){
//     println!("Hello, world!");
//     for i in 0..10{
//         println!("Number: {}", i);
//     }
// }

fn main(){
    let mut count =0;
    println!("lets count untill infinity");
    //infinity loop
    loop{
        count += 1;
        if count == 3{
            println!("three");
            continue;
        }
        println!("{}", count);
        if count == 5{
            println!("ok thats enough");
            break;
        }


        
    }
}