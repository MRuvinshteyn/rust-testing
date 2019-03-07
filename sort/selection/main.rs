fn selection_sort(v:&mut Vec<i64>) -> () {
    for i in 0..(v.len() - 1) {
        let mut min:i64 = v[i + 1];
        let mut min_pos:usize = i + 1;
        for j in i..(v.len() - 1) {
            if v[j] < min {
                min = v[j];
                min_pos = j;
            }
        }
        v[min_pos] = v[i];
        v[i] = min;
    }
}

fn main() {
    let mut v:Vec<i64> = vec![5,3,4,7,6,1,2,0,9,8];
    selection_sort(&mut v);
    println!("{:?}", v);
}
