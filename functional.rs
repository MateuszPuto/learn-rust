fn main() {
    let nums = vec![0, 1, -2, 3, 4, -5, 6, 7, -8, -9];

    let new_nums: Vec<i32> = nums.into_iter().filter(|x: &i32| x > &0).collect();

    new_nums.into_iter().for_each(|num| {
        println!("{}", num);
    });
}
