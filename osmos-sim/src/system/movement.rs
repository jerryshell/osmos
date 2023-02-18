pub fn process(rng: &mut rand::rngs::ThreadRng, object_list: &mut [crate::object::Object]) {
    for object in object_list {
        object.cell.velocity += object.cell.acceleration;
        object.cell.velocity = object
            .cell
            .velocity
            .cap_magnitude(object.cell.get_velocity_max_magnitude());

        object.cell.position += object.cell.velocity;

        if !(0.0..1.0).contains(&object.cell.position.x)
            || !(0.0..1.0).contains(&object.cell.position.y)
        {
            object.cell.position.x = rand::Rng::gen_range(rng, 0.0..=1.0);
            object.cell.position.y = rand::Rng::gen_range(rng, 0.0..=1.0);
        }
    }
}
