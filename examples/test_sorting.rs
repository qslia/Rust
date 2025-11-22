use the_algorithms_rust::sorting::bubble_sort;

fn main() {
    let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
    println!("Original: {:?}", arr);
    
    // Sort the array using bubble sort
    bubble_sort(&mut arr);
    
    println!("After sorting: {:?}", arr);
}