/* The following exercises were borrowed from Will Crichton's CS 242 Rust lab. */

use std::collections::HashSet;

fn main() {
    println!("Hi! Try running \"cargo test\" to run tests.");
}

fn add_n(v: Vec<i32>, n: i32) -> Vec<i32> {
    // init vec
    let mut res = Vec::with_capacity(v.len());
    for idx in 0..v.len() {
        // without reallocating
        res.push(n + v[idx]);
    }
    return res;
}

fn add_n_inplace(v: &mut Vec<i32>, n: i32) {
    for num in v.iter_mut() {
        *num += n;
    }
}

fn dedup(v: &mut Vec<i32>) {
    let mut appear = HashSet::new();
    let mut remove_idx = Vec::new();
    for idx in 0..v.len() {
        if appear.contains(&v[idx]) {
            remove_idx.push(idx);
        } else {
            appear.insert(&v[idx]);
        }
    }
    for idx in 0..remove_idx.len() {
        v.remove(remove_idx[idx] - idx as usize);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_n() {
        assert_eq!(add_n(vec![1], 2), vec![3]);
    }

    #[test]
    fn test_add_n_inplace() {
        let mut v = vec![1];
        add_n_inplace(&mut v, 2);
        assert_eq!(v, vec![3]);
    }

    #[test]
    fn test_dedup() {
        let mut v = vec![3, 1, 0, 1, 4, 4];
        dedup(&mut v);
        assert_eq!(v, vec![3, 1, 0, 4]);
    }
}
