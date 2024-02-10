#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn interator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut vector_iterator = v1.iter();

        assert_eq!(vector_iterator.next(), Some(&1));
        assert_eq!(vector_iterator.next(), Some(&2));
        assert_eq!(vector_iterator.next(), Some(&3));
        assert_eq!(vector_iterator.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3, 5, 9, 10];
        let v1_iterator = v1.iter();

        let total: i32 = v1_iterator.sum();
        assert_eq!(total, 30);
    }

    #[test]
    fn methods_that_return_iterators() {
        let v1 = vec![1, 2, 3];
        let new_v: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(new_v, vec![2, 3, 4]);
    }

    #[test]
    fn test_shoe_in_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let my_fit_shoes = shoes_in_size(shoes, 10);

        assert_eq!(
            my_fit_shoes,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        );
    }
}
