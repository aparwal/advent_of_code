use common::file_to_vec;

fn validity_check1(password: &String) -> bool {
  let split: Vec<&str> = password.split(" ").collect();
  let min_max: Vec<&str> = split[0].split("-").collect();
  let min_count: usize = min_max[0].parse().unwrap();
  let max_count: usize = min_max[1].parse().unwrap();
  let character = &split[1][0..1];
  let count = split[2].matches(character).count();

  count >= min_count && count <= max_count
}

fn validity_check2(password: &String) -> bool {
  let split: Vec<&str> = password.split(" ").collect();
  let min_max: Vec<&str> = split[0].split("-").collect();
  let min_count: usize = min_max[0].parse().unwrap();
  let max_count: usize = min_max[1].parse().unwrap();
  let character = &split[1][0..1].chars().next().unwrap();


  (split[2].chars().nth(min_count - 1).unwrap() == *character)
    ^ (split[2].chars().nth(max_count - 1).unwrap() == *character)
}

fn part1(input_vec: &Vec<String>) -> usize {
  input_vec
    .into_iter()
    .map(|password| validity_check1(password))
    .filter(|pass| *pass)
    .count()
}

fn part2(input_vec: &Vec<String>) -> usize {
  input_vec
    .into_iter()
    .map(|password| validity_check2(password))
    .filter(|pass| *pass)
    .count()
}


fn main() {
  let lines = file_to_vec::<String>("day2/input.txt");

  println!("{}", part1(&lines));

  println!("{}", part2(&lines));

}
