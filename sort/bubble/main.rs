fn check_sort(v:Vec<i64>) -> bool {
    for i in 0..(v.len() - 1) {
        if v[i] > v[i + 1] { return false; }
    }
    true
}

fn bubble_sort(v:&mut Vec<i64>) -> () {
    let mut temp:i64;
    while !check_sort(v.to_vec()) {
        for i in 0..(v.len() - 1) {
            if v[i] > v[i + 1] {
                temp = v[i];
                v[i] = v[i + 1];
                v[i + 1] = temp;
            }
        }
    }
}

fn main() {
    let mut v:Vec<i64> = vec![5,3,4,7,6,1,2,0,9,8];
    bubble_sort(&mut v);
    println!("{:?}", v);
}
