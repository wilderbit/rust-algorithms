use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};

pub fn selection_sort<T>(list: &mut [T])
where
    T: PartialOrd + PartialEq,
{
    let n = list.len();
    for i in 0..n - 1 {
        let mut min_index = i;
        for j in i + 1..n {
            if list[min_index] > list[j] {
                min_index = j;
            }
        }
        if i != min_index {
            list.swap(i, min_index);
        }
    }
}

#[derive(Debug, Clone)]
struct Employee {
    id: i32,
    name: String,
}

impl PartialOrd for Employee {
    fn partial_cmp(&self, other: &Employee) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Employee {
    fn eq(&self, other: &Employee) -> bool {
        self.id == other.id
    }
}

impl Ord for Employee {
    fn cmp(&self, other: &Employee) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl Eq for Employee {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn selection_sort_int() {
        let mut arr = [2, 1, 3, 5, 4];
        selection_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn selection_sort_struct() {
        let mut arr = [
            Employee {
                id: 2,
                name: "Abrar".to_string(),
            },
            Employee {
                id: 1,
                name: "Khan".to_string(),
            },
        ];
        selection_sort(&mut arr);
        assert_eq!(
            arr,
            [
                Employee {
                    id: 1,
                    name: "Khan".to_string(),
                },
                Employee {
                    id: 2,
                    name: "Abrar".to_string(),
                }
            ]
        );
    }
}
