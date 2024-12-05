pub fn parse_to_vec<T>(str: &str, delim: &str) -> Result<Vec<T>, T::Err>
where
    T: std::str::FromStr,
{
    str.split(delim).map(|s| s.parse::<T>()).collect()
}

