pub mod db;
pub mod util;

pub fn test() {
    println!("Test");
}

#[allow(dead_code)]
fn binary_search(a: &[u64], num: u64) -> bool {
    let mut li = 0;
    let mut hi = a.len();

    while li < hi {
        let mid = li + ((hi - li) / 2);
        let val = a[mid];
        if val == num {
            return true;
        } else if val < num {
            li = mid + 1
        } else {
            hi = mid
        }
    }

    false
}

#[test]
fn test_bin_search() {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let len = 100;
    let mut vals: Vec<u64> = (0..len)
        .map(|_| rng.gen_range(0..2000000000))
        .collect::<Vec<u64>>();
    vals.sort();
    println!("testing for len {}", len);
    assert_eq!(binary_search(&vals[0..], 387902), vals.contains(&387902));

    let len = 10000;
    let mut vals: Vec<u64> = (0..len)
        .map(|_| rng.gen_range(0..2000000000))
        .collect::<Vec<u64>>();
    vals.sort();
    println!("testing for len {}", len);
    assert_eq!(binary_search(&vals[0..], 387902), vals.contains(&387902));
}
