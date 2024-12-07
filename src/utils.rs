use std::time::SystemTime;

pub fn parse_to_vec<T>(str: &str, delim: &str) -> Result<Vec<T>, T::Err>
where
    T: std::str::FromStr,
{
    str.split(delim).map(|s| s.parse::<T>()).collect()
}

#[allow(unused)]
pub fn timeit<F: Fn() -> T, T>(f: F) -> T {
  let start = SystemTime::now();
  let result = f();
  let end = SystemTime::now();
  let duration = end.duration_since(start).unwrap();
  println!("it took {:?}", duration);
  result
}

