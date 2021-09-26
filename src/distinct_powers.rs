use num_bigint::{BigUint, ToBigUint};

pub struct DistinctPowers {
    a_max: u32,
    b_max: u32,
    distinct_terms: Vec<BigUint>,
}

impl DistinctPowers {
    pub fn new(a_max: u32, b_max: u32) -> DistinctPowers {
        DistinctPowers {
            a_max,
            b_max,
            distinct_terms: Vec::new(),
        }
    }

    fn calc_number_of_distinct_terms(&mut self) {
        for i in 2..=self.a_max {
            for j in 2..=self.b_max {
                let num = i.to_biguint().unwrap();
                let num = num.pow(j);
                self.distinct_terms.push(num);
            }
        }
        self.distinct_terms.sort();
        self.distinct_terms.dedup();
    }

    pub fn number_of_distinct_terms(&mut self) -> usize {
        if self.distinct_terms.len() == 0 {
            self.calc_number_of_distinct_terms();
        }
        self.distinct_terms.len()
    }
}

#[cfg(test)]
mod tests {
    use super::DistinctPowers;

    #[test]
    fn number_of_distinct_terms_for_max_5() {
        let mut dp = DistinctPowers::new(5, 5);
        assert_eq!(dp.number_of_distinct_terms(), 15);
    }
}
