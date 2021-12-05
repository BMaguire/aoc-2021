

pub mod file_util {
    use std::fs;

    // read a file and return an iterable output

    pub fn read_file(filename: &str) -> Vec<String> {
        fs::read_to_string(filename).expect("Something went wrong while reading the file").split("\n").map(|a| a.to_string()).collect()
    }

    pub fn read_to_casted_vec<T: std::str::FromStr>(filename: &str) -> Vec<T>
    where T: std::str::FromStr,
         <T as std::str::FromStr>::Err: std::fmt::Debug
    {
        return read_file(filename).iter().map(|s| s.parse::<T>().unwrap() ).collect();
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
