//! A = (a, b, c) B = (d, e) C = (f, g)
//!
//! | index | A-pos | B-pos | C-pos |
//! |-------|-------|-------|-------|
//! | 0     | 0     | 0     | 0     |
//! | 1     | 1     | 0     | 0     |
//! | 2     | 2     | 0     | 0     |
//! | 3     | 0     | 1     | 0     |
//! | 4     | 1     | 1     | 0     |
//! | 5     | 2     | 1     | 0     |
//! | 6     | 0     | 0     | 1     |
//! | 7     | 1     | 0     | 1     |
//! | 8     | 2     | 0     | 1     |
//! | 9     | 0     | 1     | 1     |
//! | 10    | 1     | 1     | 1     |
//! | 11    | 2     | 1     | 1     |

const N: usize = 12;
type Pos = Vec<Vec<usize>>;

fn helper(len: Vec<usize>) -> Pos {
    let total = len.iter().product();
    let mut it: Vec<Vec<usize>> = Vec::with_capacity(len.len());
    len.iter().fold(1, |acc, l| {
                  let f = |n| n / acc % l;
                  it.push((0..total).map(f).collect());
                  acc * l
              });
    it
}

#[test]
fn iter_3_2_2() {
    let pos = helper(vec![3, 2, 2]);
    let desired = &[&[0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2],
                    &[0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1],
                    &[0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1]];
    assert_eq!(pos, desired);
}

#[test]
fn base() {
    // let v: Vec<_> = (0..3).cycle().take(N).collect();
    let v: Vec<_> = (0..N).map(|n| n % 3).collect();
    assert_eq!(v, [0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2,]);
}

#[test]
fn base1() {
    let v: Vec<_> = (0..N).map(|n| n / 3 % 2).collect();
    assert_eq!(v, [0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1]);
}

#[test]
fn base2() {
    let v: Vec<_> = (0..N).map(|n| n / 6 % 2).collect();
    assert_eq!(v, [0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn iter_1() {
    let pos = helper(vec![1]);
    assert_eq!(pos, &[&[0]]);
}

#[test]
fn iter_3() {
    let pos = helper(vec![3]);
    assert_eq!(pos, &[[0, 1, 2]]);
}

#[test]
fn iter_1_1() {
    let pos = helper(vec![1, 1]);
    assert_eq!(pos, &[[0], [0]]);
}

#[test]
fn iter_1_2() {
    let pos = helper(vec![1, 2]);
    assert_eq!(pos, &[[0, 0], [0, 1]]);
}

#[test]
fn iter_3_1() {
    let pos = helper(vec![3, 1]);
    assert_eq!(pos, &[[0, 1, 2], [0, 0, 0]]);
}

#[test]
fn iter_2_4() {
    let pos = helper(vec![2, 4]);
    assert_eq!(pos, &[[0, 1, 0, 1, 0, 1, 0, 1], [0, 0, 1, 1, 2, 2, 3, 3]]);
}

#[test]
fn iter_0() {
    let pos = helper(vec![]);
    let s: &[&[usize]] = &[];
    assert_eq!(pos, s);
    let pos = helper(vec![0]);
    assert_eq!(pos, &[&[]]);
    let pos = helper(vec![0, 0]);
    assert_eq!(pos, &[&[], &[]]);
    let pos = helper(vec![0, 0, 0]);
    assert_eq!(pos, &[&[], &[], &[],]);
}

fn transpose(v: Vec<Vec<usize>>, len: usize) -> Vec<Vec<usize>> {
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len).map(|_| iters.iter_mut().map_while(|n| n.next()).collect()).collect()
}

#[test]
fn transpose_1_2() {
    let v = vec![vec![1, 2]];
    assert_eq!(transpose(v, 2), &[&[1], &[2]]);
}

#[test]
fn transpose_3_2() {
    let v = vec![vec![1, 2], vec![3, 4], vec![5, 6],];
    assert_eq!(transpose(v, 2), &[&[1, 3, 5], &[2, 4, 6]]);
}
