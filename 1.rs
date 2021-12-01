use std::fs;
//use fs::File;


fn main() {
    let arr: Vec<i64> = input().lines().map(|str| str.to_string().parse::<i64>().unwrap()).collect();
    println!("Part one \n{}",part_one(arr.clone()));
    println!("Part two \n{}",part_two(arr));


}


fn part_one(arr:Vec<i64>) -> i64{
    let mut ans: i64 = 0;
    for i in 1..arr.len(){
        if arr[i] > arr[i-1]{
            ans+=1;
        }
    }
    ans
    
}
fn part_two(arr:Vec<i64>) -> i64{
    let mut last_ans: i64 =  arr[0] + arr[1] + arr[2];
    let mut ans: i64 = 0;
    for i in 2..arr.len()-1{
        if  last_ans  <  last_ans - arr[i-2] + arr[i+1]{
            ans+=1;
            last_ans += arr[i+1] - arr[i-2];
        }
    }
    ans
    
}

fn input() -> String
{
    fs::read_to_string("in").unwrap()
}
