pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = AveragedCollection {
            list: vec![1, 2, 3],
            average: 2.0
        };

        assert_eq!(a.average, 2.0);
    }

    #[test]
    fn test_get_average() {
        let a = AveragedCollection {
            list: vec![1, 2, 3],
            average: 2.0
        };

        assert_eq!(a.average, 2.0);
    }

    #[test]
    fn test_add() {
        let mut a = AveragedCollection {
            list: vec![1, 2, 3],
            average: 2.0
        };

        a.add(6);

        assert_eq!(a.average, 3.0);
    }

    #[test]
    fn test_remove() {
        let mut a = AveragedCollection {
            list: vec![1, 2, 3, 6],
            average: 3.0
        };

        a.remove();

        assert_eq!(a.average, 2.0);
    }
}
