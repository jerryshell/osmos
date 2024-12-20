pub type GeneList = Vec<f64>;

pub trait GeneObject {
    fn build(
        rng: &mut rand::rngs::ThreadRng,
        gene_list: GeneList,
        id: usize,
        max_x: f64,
        max_y: f64,
    ) -> Self;

    fn get_gene_list(&self) -> GeneList;

    fn fitness(&self) -> isize;
}
