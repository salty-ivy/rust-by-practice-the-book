
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

fn greeting(name: &str) -> String {
    format!("hello {}", name)
    // String::from("hello") // intentional failing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle {
            width: 9,
            height: 10,
        };

        let smaller = Rectangle {
            width: 4,
            height: 6,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn greeting_contains_name(){
        let result = greeting("carol");
        assert!(
            result.contains("carol"),
            "Greetings did not contain name, value was `{}`", result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")] // expected checks if the panic message contains the subsring  or not
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String>{
        if 2+2 == 4 {
            Ok(())
        }else {
            Err(String::from("tow plus two does not equal four"))
        }
    }

    #[test]
    #[ignore]
    fn check_assert_on_result() {
        let ok: Result<(), String> = Err(String::from("some error explanantion"));
        println!("yooyyoyoyoyyoyoyo{}", ok.is_err());
        assert!(ok.is_err());
    }

}
