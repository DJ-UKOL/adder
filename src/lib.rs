mod rectangle;
mod guess;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: u64) -> u64 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

#[cfg(test)]            // атрибут указывает, что это тестовая функция
mod tests {
    use crate::rectangle::Rectangle;
    use crate::guess::Guess;
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn another() {
        //panic!("Make this test fail");
    }

    #[test]
    fn large_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_large() {
        let large = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&large));
    }

    #[test]
    fn it_add_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contains name, value was '{result}'"
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")] // атрибут сообщает системе тестирования, что тест проходит, когда метод генерирует ошибку
    fn greater_than_100 () {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())      // возвращаем Ok(()) когда тест успешно выполнен
        } else {
            Err(String::from("two plus two does not equal four"))   // Err со String внутри, когда тест не проходит
        }
    }
}
