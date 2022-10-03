fn main() {

    // let mut t = ("hello",true, 3);

    // let mut l = [1,2,3];
    // for item in l {
    //     println!("> {}", item)
    // }

    let vec = vec![1,2,4];
    for data in vec.iter() {
        if *data == 2 {
            println!("/ {}", data)
        }
    }

}
