fn combine(a:Vec<i64>, b:Vec<i64>) -> Vec<i64> {
    let mut ret:Vec<i64> = Vec::new();
    let mut i:usize = 0;
    let mut j:usize = 0;
    while i + j < a.len() + b.len() {
        if i == a.len() {
            ret.push(b[j]);
            j += 1;
        }
        else if j == b.len() {
            ret.push(a[i]);
            i += 1;
        }
        else if a[i] < b[j] {
            ret.push(a[i]);
            i += 1;
        }
        else {
            ret.push(b[j]);
            j += 1;
        }
    }
    ret
}

fn merge_sort_helper(v:Vec<i64>) -> Vec<i64> {
    if v.len() <= 1 { return v; }
    let mut lower:Vec<i64> = Vec::new();
    let mut upper:Vec<i64> = Vec::new();
    let half:usize = v.len() / 2;
    for i in 0..v.len() {
        if i < half { lower.push(v[i]); }
        else { upper.push(v[i]); }
    }
    combine(merge_sort_helper(lower), merge_sort_helper(upper))
}

fn merge_sort(v:&mut Vec<i64>) -> () {
    *v = merge_sort_helper(v.to_vec());
}

fn main() {
    let mut v:Vec<i64> = vec![5,3,4,7,6,1,2,0,9,8];
    merge_sort(&mut v);
    println!("{:?}", v);
}