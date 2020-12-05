use common::file_to_vec_split_at;

fn validate_passport(passport: &String) -> bool {
  let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
  let a: Vec<&str> = passport
    .split_whitespace()
    .map(|keyval| keyval.split(":").next())
    .flatten()
    .collect();

  required_fields.iter().all(|field| a.contains(&field))
}

fn part1(input_vec: &Vec<String>) -> usize {
  input_vec
    .iter()
    .map(|pass| validate_passport(pass))
    .filter(|pass| *pass)
    .count()
}

fn main() {
  let lines = file_to_vec_split_at::<String>("day4/input.txt", "\n\n");

  println!("{}", part1(&lines));
}
