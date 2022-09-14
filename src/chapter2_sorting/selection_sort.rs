use std::time::SystemTime;

use super::{gen_seq, is_sorted};

fn selection_sort<T: Ord>(seq: &mut [T]) {
    for i in 0..seq.len() {
        let mut min = i;
        for j in i + 1..seq.len() {
            if seq[j] < seq[min] {
                min = j;
            }
        }
        seq.swap(i, min);
    }
}

#[test]
fn test() {
    let size = 20_000;
    let mut seq = gen_seq(size);
    let begin = SystemTime::now();
    selection_sort(&mut seq);
    let time = SystemTime::now().duration_since(begin).unwrap_or_default();
    println!(
        "{} random number Selection Sort used {} secs.",
        size,
        time.as_secs_f32()
    );
    assert!(is_sorted(&seq));
}
