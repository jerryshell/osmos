pub fn process(object_list: &mut [crate::object::Object]) {
    object_list.iter_mut().for_each(|object| {
        object.cell.velocity = object.cell.direction * object.cell.get_speed();
        object.cell.position += object.cell.velocity;
        object.cell.position.x = nalgebra::wrap(object.cell.position.x, 0.0, 1.0);
        object.cell.position.y = nalgebra::wrap(object.cell.position.y, 0.0, 1.0);
    });
}
