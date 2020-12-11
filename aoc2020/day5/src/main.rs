use common::file_to_vec;

fn get_id(seat_str: &String) -> usize {
  let binary: String = seat_str
    .chars()
    .map(|x| match x {
      'F' => '0',
      'B' => '1',
      'R' => '1',
      'L' => '0',
      _ => 'x',
    })
    .collect();

  usize::from_str_radix(&(binary[..7]), 2).unwrap() * 8
    + usize::from_str_radix(&(binary[7..]), 2).unwrap()
}

fn part1(input_vec: &Vec<String>) -> usize {
  *input_vec
    .iter()
    .map(|pass| get_id(pass))
    .collect::<Vec<usize>>()
    .iter()
    .max()
    .unwrap()
}

fn part2(input_vec: &Vec<String>) -> usize {
  let mut sorted_seats: Vec<usize> = input_vec
    .iter()
    .map(|pass| get_id(pass))
    .collect::<Vec<usize>>();
  sorted_seats.sort();
  let mut last = sorted_seats[0] - 1;
  for elem in sorted_seats.iter() {
    if *elem - 1 != last {
      return *elem - 1;
    }
    last = *elem;
  }
  0
}

fn main() {
  let lines = file_to_vec::<String>("day5/input.txt");

  println!("{}", part1(&lines));
  println!("{}", part2(&lines));
}
