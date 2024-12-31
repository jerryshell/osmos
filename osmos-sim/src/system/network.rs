pub fn process(object_list: &mut [crate::object::Object]) {
    object_list.iter_mut().for_each(|object| {
        let nn_output = object.network.feed(&object.cell.sensor.data_list);
        object.network_output = nn_output[0];
        object.cell.direction.x = nn_output[0].cos();
        object.cell.direction.y = nn_output[0].sin();
    });
}
