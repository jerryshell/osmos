pub fn process(object_list: &mut [crate::object::Object]) {
    for object in object_list {
        // if object.cell.energy == 1 {
        //     object.cell.acceleration = nalgebra::Vector2::new(0.0, 0.0);
        // }
        object.cell.velocity += object.cell.acceleration;
        object.cell.velocity = object
            .cell
            .velocity
            .cap_magnitude(object.cell.velocity_max_magnitude);

        object.cell.position += object.cell.velocity;
        object.cell.position.x = nalgebra::wrap(object.cell.position.x, 0.0, 1.0);
        object.cell.position.y = nalgebra::wrap(object.cell.position.y, 0.0, 1.0);
    }
}
