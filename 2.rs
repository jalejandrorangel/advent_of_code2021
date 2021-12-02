use std::fs;

fn main(){

    let data: Vec<(String, i64)> = input();
    part_one(data.clone());
    part_two(data);
}

fn part_two(data: Vec<(String, i64)>){
    //down X increases your aim by X units.
    //up X decreases your aim by X units.
    //forward X does two things:
    //    It increases your horizontal position by X units.
    //    It increases your depth by your aim multiplied by X.
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for i in 0..data.len(){
        if data[i].0 == "forward"{
            horizontal += data[i].1;
            depth += aim * data[i].1;
        }
        else if data[i].0 == "down"{
            aim += data[i].1;
        }
        else{
            aim -= data[i].1;
        }
    }
    println!("Part two {} ", horizontal * depth);
}

fn part_one(data: Vec<(String,i64)>){
    //forward X increases the horizontal position by X units.
    //down X increases the depth by X units.
    //up X decreases the depth by X units.
    let mut horizontal = 0;
    let mut depth = 0;
    for i in 0..data.len(){
        if data[i].0 == "forward"{
            horizontal += data[i].1;
        }
        else if data[i].0 == "down"{
            depth += data[i].1;
        }
        else{
            depth -= data[i].1;
        }
    }
    println!("Part one {}", horizontal * depth);
}

fn input() -> Vec<(String, i64)>
{
    let doc = fs::read_to_string("in").unwrap();
    let data: Vec<(String, i64)>  = doc.lines().map(|str| str.to_string())
                                                .into_iter()
                                                .map(|pos| {
                                                    let mut iter = pos.split_whitespace();
                                                    return (
                                                        iter.next().unwrap().to_string(),
                                                        iter.next().unwrap().parse::<i64>().unwrap()
                                                    )}
                                                ).collect(); 
    data
}
