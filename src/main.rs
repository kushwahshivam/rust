fn main() {
    let n = fibo(4);
    let m = is_even(20);
    println!("{}",n);
    println!("{}",m);
    

    let name:&str = "shivam";
    println!("{}",name);    
}



fn fibo(num : i32)->i32{
    let mut first = 0;
    let mut second =1;
    if num==0{
        return first;
    }
    if num==1{
        return 1;
    }
    for _ in 1..num-1{
        let temp = second;
        second = second + first;
        first= temp;
    }
    return second;
}

fn is_even(num : i32)->bool{
    if num%2==0{
        return true;
    }
    return false;
}