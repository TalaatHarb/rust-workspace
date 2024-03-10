pub fn calculate_weight_on_mars(weight: &f32) -> f32 {
    return 3.711f32 * weight / 9.81;
}

#[cfg(test)]
mod tests {
    use crate::calc::calculate_weight_on_mars;
    use rand::Rng;

    fn precision_f32(x: f32, digits_of_percision: usize) -> f32 {
        if x == 0. || digits_of_percision == 0 {
            0.
        } else {
            let shift: i32 = digits_of_percision as i32 - x.abs().log10().ceil() as i32;
            let shift_factor: f32 = 10_f32.powi(shift);

            (x * shift_factor).round() / shift_factor
        }
    }

    #[test]
    fn test_calculate_weight_on_mar_when_weight_zero() {
        let weight: f32 = 0.0f32;
        assert_eq!(weight, calculate_weight_on_mars(&weight));
    }

    #[test]
    fn test_calculate_weight_on_mar_when_weight_100() {
        let weight_on_earth: f32 = 100.0f32;
        let expected_on_mars: f32 = 37.828747f32;
        assert_eq!(expected_on_mars, calculate_weight_on_mars(&weight_on_earth));
    }

    #[test]
    fn test_calculate_weight_on_mar_is_linear() {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

        // random weights on earth
        let we1: f32 = rng.gen_range(0.0f32..100.0f32);
        let we2: f32 = rng.gen_range(0.0f32..100.0f32);
        let we: f32 = we1 + we2;

        // the equivalent weights on mars
        let wm1: f32 = calculate_weight_on_mars(&we1);
        let wm2: f32 = calculate_weight_on_mars(&we2);

        // sums are consistant to 3 digit percision
        let expected_wm: f32 = precision_f32(wm1 + wm2, 3);
        let wm: f32 = precision_f32(calculate_weight_on_mars(&we), 3);

        assert_eq!(expected_wm, wm);
    }
}
