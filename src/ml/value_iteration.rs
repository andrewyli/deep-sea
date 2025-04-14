use num::Num;
use std::iter::Sum;
use std::ops::Mul;


pub struct ApproximateQState<'a, S, A, T, U> {
    weights: Vec<U>,
    features: Vec<Feature<'a, S, A, T>>,
}


type FeatureFn<'a, S, A, T> = dyn Fn(&S, &A) -> T + 'a;
pub struct Feature<'a, S, A, T> {
    value_fn: Box<FeatureFn<'a, S, A, T>>,
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
    fn new(value_fn: Box<FeatureFn<'a, S, A, T>>) -> Self
    where
        S: 'a, A: 'a, T: 'a
    {
        Self {
            value_fn,
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

    use crate::{
        deep_sea::{DeepSea, DiveDirection},
        deep_sea_vectorization::{DeepSeaAction, DeepSeaState, DeepSeaStateActionPair},
        ml::{value_iteration::Feature, vectorization::{IntoTensorData, Unpackable}},
        solver::TreasureDecision};

    use burn::tensor::Tensor;
    use burn_ndarray::{NdArray, NdArrayDevice};

    #[gtest]
    fn test_deep_sea_feature() {
        let oxygen_feature: Feature<'_, DeepSea, DeepSeaAction, f32> = Feature::new(
            Box::new(|s: &DeepSea, _a: &DeepSeaAction| s.oxygen() as f32)
        );
        let distance_feature: Feature<'_, DeepSea, DeepSeaAction, f32> = Feature::new(
            Box::new(|s: &DeepSea, _a: &DeepSeaAction| s.player_idx() as f32)
        );
        let deep_sea = DeepSea::new(Vec::new(), 1);
        let deep_sea_action = DeepSeaAction::DiveDirection(DiveDirection::Down);
        expect_eq!(DeepSea::OXYGEN as f32, oxygen_feature.evaluate(&deep_sea, &deep_sea_action));
        expect_eq!(0.0, distance_feature.evaluate(&deep_sea, &deep_sea_action));
    }

    #[gtest]
    fn test_sum_of_tiles_feature() {
        let state_action_feature = Feature::new(
            Box::new(
                |state: &DeepSeaState, _action: &DeepSeaAction| state.path.into_tensordata::<f32>())
        );
        // Not a realistic feature.
        let sum_feature = Feature::new(
            Box::new(
                |state: &DeepSeaState, action: &DeepSeaAction|
                Tensor::<NdArray, 1>::from_data(
                    state_action_feature.evaluate(state, action),
                    &NdArrayDevice::Cpu).sum()
            )
        );
        let state = DeepSeaState::default();
        let action = DeepSeaAction::TreasureDecision(TreasureDecision::Take);
        expect_eq!(
            sum_feature
                .evaluate(&state, &action)
                .to_data()
                .to_vec::<u16>()
                .unwrap()[0] as usize,
            8 * (1 + 2 + 3 + 4) + state.players.len() * (1 + action.unpack::<u16>().next().unwrap() as usize) + DeepSea::OXYGEN as usize
        );
    }
}
