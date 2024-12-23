pub type Gene = Vec<f32>;

pub trait GeneObject {
    fn build(rng: &mut impl rand::RngCore, gene: Gene, id: usize) -> Self;

    fn gene(&self) -> Gene;

    fn fitness(&self) -> isize;
}
