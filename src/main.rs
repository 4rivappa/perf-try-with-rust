fn run_benchmark(size: u32) {
    let mut vec = random_vector(size);

    quick_sort(&mut vec);
    assert_sorted(&vec);
    println!("completed quicksort for {} count...", size);
}

fn quick_sort(slice: &mut [i32]) {
    if !slice.is_empty() {
        let partition_index = partition(slice);
        let len = slice.len();

        quick_sort(&mut slice[0..partition_index]);
        quick_sort(&mut slice[partition_index + 1..len]);
        assert_sorted(slice);
    }
}

fn partition(slice: &mut [i32]) -> usize {
    let len = slice.len();
    let pivot = slice[len - 1];
    let mut i = 0;
    let mut j = 0;

    while j < len - 1 {
        if slice[j] <= pivot {
            slice.swap(i, j);
            i += 1;
        }
        j += 1;
    }

    slice.swap(i, len - 1);

    i
}

fn assert_sorted(slice: &[i32]) {
    for i in 1..slice.len() {
        assert!(slice[i - 1] <= slice[i])
    }
}

fn random_vector(size: u32) -> Vec<i32> {
    let mut slice: Vec<i32> = Vec::new();
    for _ in 0..size {
        slice.push(rand::random::<i32>());
    }
    slice
}

fn main() {
    run_benchmark(100);
}
