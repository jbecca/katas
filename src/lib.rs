mod db;

use rand::prelude::*;

pub fn test() {
    println!("Test");
    db::test();
}

fn linear_search(a: &[u32], num: u32) -> bool {
    for element in 0..a.len() {
        if a[element] == num {return true};
    }

    false
}

fn binary_search(a: &[u64], num: u64) -> bool {
    let mut li = 0;
    let mut hi = a.len();

    while li < hi {
        let mid = li + ((hi - li) / 2);
        let val = a[mid];
        if val == num {
            return true
        } else if val < num {
            li = mid + 1
        } else {
            hi = mid
        }
    }

    false
}

#[test]
fn test_lin_search() {
    assert_eq!(linear_search(&[1,2,3,4,5], 4), true);
    assert_eq!(linear_search(&[1,2,3,4,5], 6), false);
    assert_eq!(linear_search(&[1,2,3,4,5], 0), false);
}


#[test]
fn test_bin_search() {
    let mut rng = rand::thread_rng();

    let mut vals: Vec<u64> = (0..100000).map(|_| rng.gen_range(0..2000000000)).collect::<Vec<u64>>();
    vals.sort();
    assert_eq!(binary_search(&[1,2,3,4,5], 4), true);
    assert_eq!(binary_search(&[1,2,3,4,5], 6), false);
    assert_eq!(binary_search(&[1,2,3,4,5], 0), false);
    println!("Now the long test");
    assert_eq!(binary_search(&vals[0..], 387902), vals.contains(&387902));
}
