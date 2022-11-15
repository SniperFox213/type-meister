pub fn create_linear_numbers_array(mut from: usize, to: usize) -> Vec<usize> {
  let mut numbers = Vec::<usize>::new();
  
  while from <= to {
    numbers.push(from);
    from += 1;
  };

  numbers
}