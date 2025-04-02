pub fn arr(a: [i32; 50], n: i32, t: usize) -> ([i32; 50], usize) {
    println!("");

    let mut feed: [i32; 50] = [50; 50];
    let mut ind: usize = 0;

    for index in 0..t {
       

        if a[index] != n  {
            feed[ind] = a[index];
            ind += 1;
            continue;
        } else {
            continue;
        }
    
    }


    (feed, ind)
}
