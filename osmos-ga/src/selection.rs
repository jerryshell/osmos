pub fn selection(
    rng: &mut impl rand::RngCore,
    object_list: &[impl crate::gene::GeneObject],
) -> usize {
    rand::Rng::gen_range(rng, 0..object_list.len())
}
