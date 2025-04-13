use rand::{Rng, distr::Uniform};

pub(in crate::andrew) struct Die {
    values: Vec<i32>
}

impl Die {
    pub fn roll(&self) -> i32 {
        let mut rng = rand::rng();
        self.values[rng.sample(Uniform::new(0, self.values.len()).unwrap())]
    }

    pub fn mean(&self) -> f32 {
        self.values.iter().sum::<i32>() as f32 / self.values.len() as f32
    }

    pub fn var(&self) -> f32 {
        let mean = self.mean();
        self.values.iter().map(|&v| (v as f32 - mean).powi(2)).sum::<f32>() / self.values.len() as f32
    }

    pub fn from_vec(values: Vec<i32>) -> Die {
        Die { values }
    }
}

pub(in crate::andrew) struct Dice {
    pub dice: Vec<Die>
}

impl Dice {
    pub fn roll(&self) -> i32 {
        self.dice.iter().map(|d| d.roll()).sum::<i32>()
    }

    pub fn mean(&self) -> f32 {
        self.dice.iter().map(|d| d.mean()).sum::<f32>()
    }

    pub fn var(&self) -> f32 {
        self.dice.iter().map(|d| d.var()).sum::<f32>()
    }

    pub fn from_vec(dice: Vec<Die>) -> Dice {
        Dice { dice }
    }
}
