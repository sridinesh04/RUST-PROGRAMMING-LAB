fn main() {
    let arr = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    let slice_a = &arr[1..3];
    println!("a. Slice of 2nd and 3rd element: {:?}", slice_a);

    let slice_b = &arr[3..];
    println!("b. Slice omitting the start index: {:?}", slice_b);

    let slice_c = &arr[..7];
    println!("c. Slice omitting the end index: {:?}", slice_c);

    let slice_d = &arr[..];
    println!("d. Slice omitting both start and end index: {:?}", slice_d);
}
