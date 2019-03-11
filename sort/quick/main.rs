fn partition(v:&mut Vec<i64>, pos:usize) -> Vec<i64> {
    if (*v).len() <= 1 { return (*v).clone(); }
    let mut t:Vec<i64> = Vec::new();
    let mut lower:Vec<i64> = Vec::new();
    let mut upper:Vec<i64> = Vec::new();
    let part:i64 = (*v)[pos];
    for i in 1..(*v).len() {
        if (*v)[i] < part { lower.push((*v)[i]); }
        else { upper.push((*v)[i]); }
    }
    t.append(&mut partition(&mut lower, 0));
    t.push(part);
    t.append(&mut partition(&mut upper, 0));
    t
}

fn quick_sort(v:&mut Vec<i64>) -> () {
    (*v) = partition(v, 0);
}

fn main() {
    let mut v:Vec<i64> = vec![1, 4, 1, 2, 7, 5, 2];
    quick_sort(&mut v);
    println!("{:?}", v);
}
