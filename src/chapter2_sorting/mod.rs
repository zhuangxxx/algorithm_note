mod selection_sort;

fn gen_seq(size: usize) -> Vec<i32> {
    let mut seq = vec![0; size];
    for i in 0..size {
        seq[i] = rand::random::<i32>();
    }

    seq
}

fn is_sorted<T: Ord>(seq: &[T]) -> bool {
    if seq.len() < 2 {
        return true;
    }

    for i in 1..seq.len() {
        if seq[i - 1] > seq[i] {
            return false;
        }
    }

    true
}
