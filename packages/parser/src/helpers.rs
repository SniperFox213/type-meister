use core::ops::Range;

pub fn create_linear_numbers_array(mut from: usize, to: usize) -> Vec<usize> {
	let mut numbers = Vec::<usize>::new();

	while from <= to {
		numbers.push(from);
		from += 1;
	}

	numbers
}

pub fn get_slice_from_source(source: &str, span: Range<usize>) -> String {
	let mut chars = Vec::<char>::new();

	for (i, char) in source.clone().chars().into_iter().enumerate() {
		if i >= span.start && i < span.end {
			chars.push(char);
		};
	}

	chars.into_iter().collect()
}
