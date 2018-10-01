use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};

pub fn bubble_sort<T>(list: &mut [T])
    where
        T: PartialOrd + PartialEq + Clone + ?Sized
{
    let n = list.len();
    for i in 0..n - 1 {
        let mut flag: bool = true;
        for j in 0..n - i - 1 {
            if list[j] > list[j + 1] {
                let temp = list[j].clone();
                list[j] = list[j + 1].clone();
                list[j + 1] = temp;
                flag = false;
            }
        }
        if flag {
            break;
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
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_int() {
        let mut arr = [2, 1, 3, 5, 4];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn bubble_sort_struct() {
        let mut arr = [
            Employee {
                id: 2,
                name: "Abrar".to_string(),
            },
            Employee {
                id: 1,
                name: "Khan".to_string(),
            }
        ];
        bubble_sort(&mut arr);
        assert_eq!(arr, [
            Employee {
                id: 1,
                name: "Khan".to_string(),
            },
            Employee {
                id: 2,
                name: "Abrar".to_string(),
            }
        ])
    }
}