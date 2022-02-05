use nix::unistd;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn get_host_name() -> String {
    let mut buf = [0u8; 64];
    let hostname_cstr = unistd::gethostname(&mut buf).expect("Failed to get hostname.");
    let return_result = hostname_cstr.to_str();
    return return_result.ok().unwrap().to_owned();
}

#[cfg(test)]
mod tests {
    use crate::add_one;

    #[test]
    fn get_host_name() {
        assert_eq!(5, add_one(4));
    }
}
