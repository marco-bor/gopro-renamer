pub fn is_valid(name: &str) -> bool {
  name.ends_with(".MP4") && name.len() == 12
}

pub fn sequence(name: &str) -> &str {
  &name[4..8]
}

/**
 * Different from chapter. index starts from 0
 */
pub fn index(name: &str) -> i32 {
  match &name[2..4] {
    "PR" => 0,
    _ => match &name[0..2] {
      "GP" => name[2..4].parse::<i32>().unwrap(),
      _ => name[2..4].parse::<i32>().unwrap() - 1,
    },
  }
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn test_index() {
    assert_eq!(index("GOPR0891.mp4"), 0);
    assert_eq!(index("GOPR0892.mp4"), 0);
    assert_eq!(index("GP010892.mp4"), 1);
    assert_eq!(index("GOPR0893.mp4"), 0);
    assert_eq!(index("GP010893.mp4"), 1);
    assert_eq!(index("GP020893.mp4"), 2);
  }

  #[test]
  fn test_sequence() {
    assert_eq!(sequence("GOPR0891.mp4"), "0891");
    assert_eq!(sequence("GOPR0892.mp4"), "0892");
    assert_eq!(sequence("GP010892.mp4"), "0892");
    assert_eq!(sequence("GOPR0893.mp4"), "0893");
    assert_eq!(sequence("GP010893.mp4"), "0893");
    assert_eq!(sequence("GP020893.mp4"), "0893");
  }

  #[test]
  fn test_is_valid() {
    assert_eq!(is_valid("GOPR0891.MP4"), true);
    assert_eq!(is_valid(""), false);
    assert_eq!(is_valid("GOPR0274_001.MP4"), false);
  }
}
