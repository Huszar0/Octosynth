use rand::prelude::Distribution;
use statrs::distribution::Normal;
use statrs::statistics::Statistics;
use std::collections::HashMap;

pub struct Analyzer {
    pub distributions: HashMap<&'static str, Normal>,
}

impl Analyzer {
    pub fn new() -> Self {
        Analyzer {
            distributions: HashMap::new(),
        }
    }

    pub fn generate_value(&self, type_name: &str) -> f64 {
        let mut rand_gen = rand::thread_rng();
        let distr = self
            .distributions
            .get(type_name)
            .expect("analyzer does not have distribution for this type");
        distr.sample(&mut rand_gen)
    }

    pub fn create_distr(&mut self, type_name: &'static str, data: &[f64]) {
        let mean = data.mean();
        let variance = data.variance();
        let data_distr = Normal::new(mean, variance).expect("failed to create distribution");
        self.distributions.insert(type_name, data_distr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_from_sample() {
        let mut analyzer = Analyzer::new();
        analyzer.create_distr("test_class", &[0.0, 1.5, 2.0]);
        for _ in 0..10 {
            println!("{}", analyzer.generate_value("test_class"));
        }
        panic!()
    }
}
