pub type Gene = Vec<f64>;

pub trait GeneObject {
    fn build(
        rng: &mut rand::rngs::ThreadRng,
        gene: Gene,
        id: usize,
        max_x: f64,
        max_y: f64,
    ) -> Self;

    fn gene(&self) -> Gene;

    fn fitness(&self) -> isize;
}
