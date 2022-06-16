//Exercise 1
// Mục đích: giải quyết vấn đề ownership and borrowing không dùng clone()
// fn main() {
fn change_value(input:u32, mut output: u32) -> u32{
    if input ==1 {
        output =3;
    }
    else {
        output = 4;
    }

    output
}
// Exercise 2
// Mục đích: giải quyết vấn đề ownership và borrowing ko dùng clone()
// Các bạn có thể sửa thêm logic để đúng với mục đichs bài này là liệt kê các số nguyên tố 
fn vector_is_prime(num: u64,p: &Vec<u64>) -> bool {
    for i in p {
        if num > *i && num % i != 0 {
            return true;
        }
    }

    false
}
fn vector_prime() -> Vec<u64> {
    let mut count: u32 = 1;
    let mut num: u64 = 1;
    let mut primes: Vec<u64> = Vec::new();
    primes.push(2);
    //println!("{}", primes[0]);
    while count < 20 {
        num += 2;
        if vector_is_prime(num, &mut primes) {
            count += 1;
            primes.push(num);
        }
    }
    primes
}
//Exercise 3
// Mục đích: giải quyết vấn đề ownership and borrowing ko dùng clone()
fn max_vector(a: &Vec<i32>) -> i32 {
    let mut max:i32 = 0;
    for &i in a {
        if i > max {
            max = i;
        }
    }
    max
}
fn percent_max_vector(a: Vec<i32>) -> Vec<f32> {
    let max = max_vector(&a);
    let mut result:Vec<f32> = Vec::new();
    for i in &a {
        result.push((*i as f32)/(max as f32));
    }
    result
}
//Exercise 4
// Mục đích : giải quyết vấn đề ownership và borrowing ko dùng clone()
// Logic hiện tại đang sai (cho 1 vec -> đảo chiều vector đó)
fn test(mut a: Vec<u8>) -> (Vec<u8>, i32) {
    let mut b:Vec<u8>  = Vec::new();
    let mut c:u8 = 0;
    loop {
        if a.len() == 0 { break; }
        let d = a.pop().unwrap();
        c = c+d;
        b.push(d);
    }
    (b, c as i32)
}

fn average_vector(a: Vec<i32>) -> f32 {
    let mut sum:i32 = 0;
    
    for i in &a {
        sum = sum + i;
    }   
    (sum as f32)/(a.len() as f32)       
}

fn main() {
  let a: Vec<i32> = vec![135,245,345,445,578];
  let b = percent_max_vector(a);
    let c: Vec<i32> = vec![395,205,385,783,698];
    println!("Percent in vector equal max value {:?}", b);

    println!("*******EXERCISE 1**************");
    let x = change_value(10,20);
    println!("Result Exericse 1 is {}\n\n", x);

    println!("*******EXERCISE 2**************");
    let primes = vector_prime();
    println!("Primes is {:?}\n\n", primes);

    println!("*******EXERCISE 3**************");
    let percent_max_vector = percent_max_vector(c);
   println!("Percent in vector equal max value {:?}\n\n", percent_max_vector);
    //println!("values: {:#?}\n\n", values);
    println!("*******EXERCISE 4 **************");
    let mut v: Vec<u8> = vec![1,2,3,4,5];
    let (a, c) = test(v);
    println!("Vector  {:?} tong {}",a,c);

 
}

