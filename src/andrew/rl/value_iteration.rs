use num::Num;
use std::iter::Sum;
use std::ops::Mul;

pub struct ApproximateQState<'a, S, A, T, U> {
    weights: Vec<U>,
    features: Vec<Feature<'a, S, A, T>>,
}


pub struct Feature<'a, S, A, T> {
    value_fn: Box<dyn Fn(&S, &A) -> T + 'a>,
}

impl<'a, S, A, T, U> ApproximateQState<'a, S, A, T, U>
where T: Num + Clone + Sum + Mul<U, Output = T>, U: Clone + Copy + From<f32> {
    pub fn from_features(features: Vec<Feature<S, A, T>>) -> ApproximateQState<S, A, T, U> {
        let num_features = features.len();
        ApproximateQState {
            weights: vec![U::from(1.0f32 / num_features as f32); num_features],
            features,
        }
    }


    pub fn evaluate(&self, state: &S, action: &A) -> T {
        self.features.iter().zip(self.weights.iter())
                                   .map(|(f, &w)| f.evaluate(state, action) * w).sum::<T>()
    }
}

impl<'a, S, A, T> Feature<'a, S, A, T> {
    fn new<F>(value_fn: F) -> Self
    where
        F: Fn(&S, &A) -> T + 'a
    {
        Self {
            value_fn: Box::new(value_fn),
        }
    }

    pub fn evaluate(&self, state: &S, action: &A) -> T {
        // is it dumb to write this way? Maybe there's future functionality?
        (self.value_fn)(state, action)
    }
}


#[cfg(test)]
mod tests {
    use googletest::{
        expect_eq, gtest,
    };

    use crate::{deep_sea::DeepSea, andrew::rl::value_iteration::Feature};

    #[gtest]
    fn test_deep_sea_feature() {
        struct DeepSeaAction;
        let oxygen_feature = Feature::new(
            |s: &DeepSea, _a: &DeepSeaAction| s.oxygen()
        );
        let distance_feature = Feature::new(
            |s: &DeepSea, _a: &DeepSeaAction| s.player_idx()
        );
        let deep_sea = DeepSea::new(Vec::new(), 1);
        let deep_sea_action = DeepSeaAction {};
        expect_eq!(DeepSea::OXYGEN, oxygen_feature.evaluate(&deep_sea, &deep_sea_action));
        expect_eq!(0, distance_feature.evaluate(&deep_sea, &deep_sea_action));
    }
}
