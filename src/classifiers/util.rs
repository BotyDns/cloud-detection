pub fn diff(lhs: &Vec<f32>, rhs: &Vec<f32>) -> Vec<f32> {
    lhs.iter().zip(rhs).map(|(l, r)| l - r).collect()
}
