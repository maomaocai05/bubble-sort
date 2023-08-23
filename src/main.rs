use rand::Rng;

fn main() {
    //println!("Hello, world!");
    let  mut a:Vec<i32>= Vec::new();
    for _i in 0..10 {
        a.push(rand::thread_rng().gen_range(1..=100));
    }
    println!("生成数组");
    println!("{:?}", a);

    println!("整型排序后");
    bubble_sort_i32(&mut a);
    println!("{:?}", a);

    let  mut b:Vec<i32>= Vec::new();
    for _i in 0..10 {
        b.push(rand::thread_rng().gen_range(1..=100));
    }
    println!("生成数组");
    println!("{:?}", b);
    println!("泛型方法排序后");
    bubble_sort(&mut b);
    print!("{:?}", b);
}

fn bubble_sort_i32(arr: &mut [i32]){
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j+1);
            }
        }
    }
}

fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}