fn main() {
    let vec = vec![1, 2, 3];

    let v1: std::slice::Iter<i32> = vec.iter();

    for v in v1 {
        println!("v is {v}");
    }

    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    println!("next is {:?}", v1_iter.next());

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);

    println!("total is {total}");

    let v1: Vec<i32> = vec![1, 2, 3];
    v1.iter().map(|x| x + 1);

    println!("v1 is {:?}", v1);

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    println!("v2 is {:?}", v2);
}
