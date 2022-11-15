use core::ops::Range;

pub fn get_slice_from_source(source: &str, span: Range<usize>) -> String {
  let mut chars = Vec::<char>::new();
  
  for (i, char) in source.clone().chars().into_iter().enumerate() {
    if i >= span.start && i < span.end {
      chars.push(char);
    };
  };

  chars.into_iter().collect()
}