fn count(v:Vec<i64>, i:i64) -> i64 {
    let mut cnt:i64 = 0;
    for x in 0..v.len() {
        if v[x] == i { cnt += 1; }
    }
    cnt
}

fn counting_sort(v:&mut Vec<i64>) -> () {
    // min and max variables for range
    let mut min:i64 = (*v)[0];
    let mut max:i64 = (*v)[0];
    for i in 1..(*v).len() {
        if (*v)[i] < min { min = (*v)[i]; }
        if (*v)[i] > max { max = (*v)[i]; }
    }
    // create range vector
    let mut range:Vec<i64> = Vec::new();
    for i in min..(max + 1) {
        range.push(i);
    }
    // count each item in the range
    let mut count_vector:Vec<i64> = Vec::new();
    let mut sum:i64 = 0;
    for i in min..(max + 1) {
        sum += count(v.to_vec(), i);
        count_vector.push(sum);
    }
    // push each item based on occurrences in count vector
    let mut t:Vec<i64> = Vec::new();
    let mut curr:i64 = 0;
    for i in 0..count_vector.len() {
        for _j in curr..count_vector[i] {
            t.push(range[i]);
        }
        curr = count_vector[i];
    }
    // set original vector to temp vector
    for i in 0..t.len() {
        (*v)[i] = t[i];
    }
}

fn main() {
    let mut v:Vec<i64> = vec![1, 4, 1, 2, 7, 5, 2];
    counting_sort(&mut v);
    println!("{:?}", v);
}
