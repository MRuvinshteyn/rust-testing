fn insertion_sort(v:&mut Vec<i64>) -> () {
    let mut j:usize;
    let mut temp:i64;
    for i in (0..(v.len() - 1)).rev() {
        j = i;
        while j < v.len() - 1 && v[j] > v[j + 1] {
            temp = v[j];
            v[j] = v[j + 1];
            v[j + 1] = temp;
            j += 1;
        }
    }
}

fn main() {
    let mut v:Vec<i64> = vec![5,3,4,7,6,1,2,0,9,8];
    insertion_sort(&mut v);
    println!("{:?}", v);
}
