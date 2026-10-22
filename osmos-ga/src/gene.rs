pub type Gene = Vec<f32>;

pub trait GeneObject {
    fn build(rng: &mut impl rand::Rng, gene: Gene, id: usize) -> Self;

    fn gene(&self) -> Gene;

    fn fitness(&self) -> isize;
}
