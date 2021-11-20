fn main() {
    let tup: () = ();
    let tup2: () = ();
    println!("{}", tup == tup2);

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr2 = [1, 2, 3, 4, 5];
    let arr3 = [3; 5];
    println!("{} {} {}", arr[0], arr2[0], arr3[0]);
}
