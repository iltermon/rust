mod file1;
fn main() {
    file1::use_in_main();
}

pub fn some_function(input: i32) -> String {
    if input > 0 {
        return "Yes".to_string();
    } else {
        return "No".to_string();
    }
}

fn some_private_function() -> String {
    return "private".to_string();
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn enough() {
        assert_eq!(some_private_function(), "private");
    }
}
