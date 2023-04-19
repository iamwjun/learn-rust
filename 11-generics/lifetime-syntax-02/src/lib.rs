pub use std::fmt::{Display, format};

pub fn first_word<'a>(s: &'a str) -> &'a str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      }
  }

  &s[..]
}

pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
  if a.len() > b.len() {
    a
  } else {
    b
  } 
}

// pub fn longest<'a, 'b>(a: &'a str, b: &'b str) -> &str {
//     if a.len() > b.len() {
//       a
//     } else {
//       b
//     } 
// }


pub fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
