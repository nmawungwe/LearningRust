use std::collections::HashMap;

fn median(numbers: &mut Vec<i32>) -> i32 {

    numbers.sort();
    let mid = numbers.len()/2;
    if numbers.len() % 2 == 0 {
        mean(&vec![numbers[mid -1], numbers[mid]]) as i32
    } else {
        numbers[mid]
    }

}

fn mode(numbers: &Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    for integer in numbers {
        let count = map.entry(integer).or_insert(0);
        *count += 1;
    }

    let max_value = map.values().cloned().max().unwrap_or(0);

    map.into_iter().filter(|&(_, v)| v == max_value).map(|(&k, _)|k).collect()
}

fn mean(numbers: &Vec<i32>) -> f32 {
    let sum: i32 =  numbers.iter().sum();
    sum as f32 / numbers.len() as f32
}

fn main() {
    let mut data: Vec<i32> = vec![
        1,
        1,
        2,
        3,
        26,
        34,
        36,
        36,
        38,
        43,
        43,
        43,
        43,
        54,
        76,
        328,
        378,
        2928,
    ];

    println!("MEAN, {}", mean(&data));
    println!("MEDIAN, {}", median(&mut data));
    println!("MODE, {}", mode(&data));
}
