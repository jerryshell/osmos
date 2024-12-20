pub fn process(
    rng: &mut impl rand::RngCore,
    max_x: f64,
    max_y: f64,
    object_list: &mut [crate::object::Object],
) {
    object_list.iter_mut().for_each(|object| {
        object.cell.velocity = object.cell.direction * object.cell.get_speed();

        object.cell.position += object.cell.velocity;

        if !(0.0..max_x).contains(&object.cell.position.x)
            || !(0.0..max_y).contains(&object.cell.position.y)
        {
            object.cell.random_position(rng, max_x, max_y);
        }
    });
}
