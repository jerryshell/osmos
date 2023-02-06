pub fn process(cell_list: &mut [crate::cell::Cell]) {
    for cell in cell_list {
        if cell.energy == 1 {
            cell.acceleration = nalgebra::Vector2::new(0.0, 0.0);
        }
        cell.velocity += cell.acceleration;
        cell.velocity = cell.velocity.cap_magnitude(cell.velocity_max_magnitude);

        cell.position += cell.velocity;
        cell.position.x = nalgebra::wrap(cell.position.x, 0.0, 1.0);
        cell.position.y = nalgebra::wrap(cell.position.y, 0.0, 1.0);
    }
}
