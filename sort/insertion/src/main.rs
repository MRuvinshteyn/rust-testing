fn insertion_sort(v:&mut Vec<i64>) -> () {
    for i in 1..v.len() {
        let key:i64 = v[i as usize];
        let mut j:i64 = i as i64 - 1;
        while j >= 0 && v[j as usize] > key {
            v[(j + 1) as usize] = v[j as usize];
            j -= 1;
        }
        v[(j + 1) as usize] = key;
    }
}

fn main() {
    let mut v:Vec<i64> = vec![5,3,4,7,6,1,2,0,9,8];
    insertion_sort(&mut v);
    println!("{:?}", v);
}
