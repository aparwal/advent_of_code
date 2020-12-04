use common::file_to_vec;

fn part1(input_arr: &Vec<u64>) -> u64 {
  for value in input_arr.iter() {
    if input_arr.contains(&(2020 - value)) {
      return value * (2020 - value);
    }
  }
  return 0;
}

fn part2(input_arr: &Vec<u64>) -> u64 {
  for value in input_arr.iter() {
    for value2 in input_arr.iter() {
      let mut required = 2020 - value;
      if required > *value2 {
        required -= value2;
        if input_arr.contains(&required) {
          return value * value2 * required;
        }
      }
    }
  }
  return 0;
}

fn main() {
  let lines = file_to_vec::<u64>("day1/input.txt");

  println!("{}", part1(&lines));
  println!("{}", part2(&lines));
}
