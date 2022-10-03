fn main() {
    // variables are on default immutable
    let mut x: i8 = 10; 
    println!("x is {}", x);
    x = 5;
    println!("x is {}", x);
    let mut tf: bool = true;
    let mut text = "first text";
    println!("text is {} and boolean is {}", text, tf);
    tf = false;
    text = "second text";
    println!("text is {} and boolean is {}", text, tf);

    // tuples 
    let mut t = ("hello",2,3);
    println!("The values t are {}, {}, {}", t.0,t.1,t.2);
    t.1 = 40;
    println!("The values t are {}, {}, {}", t.0,t.1,t.2);
    // deconstruction
    let (a,_b,_c) = t;
    println!("The value of a is {}", a);

    // arrays
    let l = [1,2,3];
    println!("The values l are {}, {}, {}", l[0],l[1],l[2]);

    //vector
    let mut vec = vec![1,2,4];
    println!("Starting vector data is {:?}", vec);
    vec.push(5);
    println!("Starting vector data is {:?}", vec);
    vec.pop();
    println!("Starting vector data is {:?}", vec);

}
