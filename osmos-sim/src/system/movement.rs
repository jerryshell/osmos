pub fn process(rng: &mut rand::rngs::ThreadRng, object_list: &mut [crate::object::Object]) {
    object_list.iter_mut().for_each(|object| {
        object.cell.velocity += object.cell.acceleration;
        object.cell.velocity = object
            .cell
            .velocity
            .cap_magnitude(object.cell.get_max_velocity_magnitude());

        object.cell.position += object.cell.velocity;

        if !(0.0..object.cell.max_x).contains(&object.cell.position.x)
            || !(0.0..object.cell.max_y).contains(&object.cell.position.y)
        {
            object.cell.random_position(rng);
        }
    });
}
