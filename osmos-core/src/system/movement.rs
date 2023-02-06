pub fn process(cell_list: &mut [crate::cell::Cell]) {
    for cell in cell_list {
        cell.velocity += cell.acceleration;
        cell.velocity = nalgebra::clamp(
            cell.velocity,
            nalgebra::Vector2::new(-cell.velocity_max_magnitude, -cell.velocity_max_magnitude),
            nalgebra::Vector2::new(cell.velocity_max_magnitude, cell.velocity_max_magnitude),
        );

        cell.position += cell.velocity;
        cell.position.x = nalgebra::wrap(cell.position.x, 0.0, 1.0);
        cell.position.y = nalgebra::wrap(cell.position.y, 0.0, 1.0);
    }
}
