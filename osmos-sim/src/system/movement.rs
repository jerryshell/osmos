use rand::Rng;

pub fn process(rng: &mut impl rand::RngCore, object_list: &mut [crate::object::Object]) {
    object_list.iter_mut().for_each(|object| {
        object.cell.direction.x = object.network_output.cos() * object.cell.get_speed();
        object.cell.direction.y = object.network_output.sin() * object.cell.get_speed();

        object.cell.velocity = object.cell.direction;

        object.cell.position += object.cell.velocity;

        if object.cell.position.x < 0.0
            || object.cell.position.x > 1.0
            || object.cell.position.y < 0.0
            || object.cell.position.y > 1.0
        {
            object.cell.position.x = rng.gen();
            object.cell.position.y = rng.gen();
        }
    });
}
