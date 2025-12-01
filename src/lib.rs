use std::{fs::read_to_string, str::FromStr, *};

pub fn read_input(name: &str) -> String {
    read_to_string(format!("resources/{}", name)).unwrap()
}

pub trait Extract {
    fn extract(regex: &regex::Regex, input: &str) -> Self;
}

macro_rules! extract_template {
    ($( $ti:ident ),+) => {
        #[allow(non_snake_case, dead_code)]
        impl<$($ti : std::str::FromStr),+> Extract for ($($ti),+) {
          fn extract(re: &regex::Regex, input: &str) -> Self {
            use paste::paste;
            let (_, [ $(paste!([<v _ $ti>])),+ ]) = re.captures(input).unwrap().extract();
            let ($(paste!([<r _ $ti>])),+) = (
              $(
                {
                $ti::from_str(paste!([<v _ $ti>])).map_err(|_| std::stringify!(cannot unwrap $ti)).unwrap()
                }
              ),+
            );
            ($(paste!([<r _ $ti>])),+)
          }
        }
    };
}

pub fn extract<T: FromStr>(regex: &str, input: &str) -> T {
    use regex::Regex;
    let re = Regex::new(regex).unwrap();
    let (_, [s1]) = re.captures(input).unwrap().extract();
    T::from_str(s1).map_err(|_| "cannot unwrap T").unwrap()
}

extract_template!(T1, T2);
extract_template!(T1, T2, T3);
extract_template!(T1, T2, T3, T4);
extract_template!(T1, T2, T3, T4, T5);
extract_template!(T1, T2, T3, T4, T5, T6);
extract_template!(T1, T2, T3, T4, T5, T6, T7);
extract_template!(T1, T2, T3, T4, T5, T6, T7, T8);
extract_template!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
extract_template!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
extract_template!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
extract_template!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extract_template() {
        let i: i32 = extract(r"(\d+)", "1234");
        assert_eq!(i, 1234);
    }

    #[test]
    fn extract_template_2() {
        let (i, f, s) = <(i32, f32, String)>::extract(
            &(r"integer:\s*?(\d+)\s*?float:\s*?(\d+\.\d+)string:([a-zA-Z]+)".try_into().unwrap()),
            "integer:33    float:   100.001string:abcd",
        );
        assert_eq!(i, 33);
        assert_eq!(f, 100.001);
        assert_eq!(s, "abcd");
    }
}
