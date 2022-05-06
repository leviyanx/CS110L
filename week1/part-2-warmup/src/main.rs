/* The following exercises were borrowed from Will Crichton's CS 242 Rust lab. */

use std::collections::HashSet;

fn main() {
    println!("Hi! Try running \"cargo test\" to run tests.");
}

/* return a new vector whose elements are the numbers in the original vector v with n added to each
 * number.
 */
fn add_n(v: Vec<i32>, n: i32) -> Vec<i32> {
    let mut v1: Vec<i32> = Vec::new();
    for i in v.iter() {
        v1.push(i + n)
    }
    v1
}

/* This function does the same thing as add_n, but modifies v directly (in place) and does not return anything.
 * */
fn add_n_inplace(v: &mut Vec<i32>, n: i32) {
    let mut i = 0;
    while i < v.len() {
        v[i] = v[i] + n;
        i += 1;
    }
}

/* removes duplicate elements from a vector in-place */
fn dedup(v: &mut Vec<i32>) {
    let mut hashSet: HashSet<i32> = HashSet::new(); // check if value is duplicate
    let mut insert_result;
    let mut v1: Vec<i32> = Vec::new(); // retain not duplicate value

    // Use hashset to check if item is duplicate. 
    // If item can be inserted into hashset, means it is not duplicate. 
    // So push it into new vec which retain not duplicate value.
    let mut i = 0;
    while i < v.len() {
        insert_result = hashSet.insert(v[i]);
        if insert_result == true {
            v1.push(v[i])
        }
        i += 1;
    }

    // Transfer value from new vec to old vec
    v.clear();
    for item in v1.iter() {
        v.push(*item);
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
