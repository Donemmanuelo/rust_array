use std::io;
pub fn input() -> ([i32; 50], usize, i32) {
    println!("please the number you want in your array");
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("failed to read input");
    let num: usize = num.trim().parse().expect("invalid input");
    let mut array: [i32; 50] = [0; 50];
  
    for i in 0..num {
        println!("please element of the array");
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("failed to read input");
        let temp: i32 = temp.trim().parse().expect("invalid input");
        array[i] = temp;
    }

    println!("please input the number you want to test");
    let mut tmp = String::new();
    io::stdin()
        .read_line(&mut tmp)
        .expect("failed to read input");
    let tmp: i32 = tmp.trim().parse().expect("invalid input");

    (array, num, tmp)
}
