use common::file_to_vec;

fn tree_count(map: &Vec<String>, slope: (usize, usize)) -> u64 {
  let (cloumn_offset, row_offset) = slope;
  let column_total = map[0].len();
  let mut count_tree = 0;
  let mut column = 0;
  for row in map.iter().step_by(row_offset) {
    if row.chars().nth(column % column_total).unwrap() == '#' {
      count_tree += 1;
    }
    column += cloumn_offset;
  }
  count_tree
}

fn part1(map: &Vec<String>) -> u64 {
  tree_count(map, (3, 1))
}

fn part2(map: &Vec<String>) -> u64 {
  let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
  slopes
    .iter()
    .map(|slopse| tree_count(map, *slopse))
    .product()
}

fn main() {
  let lines = file_to_vec::<String>("day3/input.txt");

  println!("{}", part1(&lines));

  println!("{}", part2(&lines));
}
