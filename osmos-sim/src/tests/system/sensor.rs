#[test]
fn danger_rd_test() {
    let mut rng = rand::thread_rng();
    let mut object_1 = crate::object::Object::new(&mut rng, 1, 1.0, 1.0);
    object_1.cell.position.x = 0.0;
    object_1.cell.position.y = 0.0;
    object_1.cell.energy = 1;
    let mut object_2 = crate::object::Object::new(&mut rng, 2, 1.0, 1.0);
    object_2.cell.position.x = 0.1;
    object_2.cell.position.y = 0.1;
    object_2.cell.energy = 2;
    let mut object_list = [object_1, object_2];
    crate::system::sensor::process(&mut object_list);
    assert_eq!(
        object_list[0].cell.sensor.data_list,
        [0.0, -14.142135623730947, -14.142135623730947, 0.0]
    );
    assert_eq!(
        object_list[1].cell.sensor.data_list,
        [7.071067811865474, 0.0, 0.0, 7.071067811865474]
    );
}

#[test]
fn danger_lu_test() {
    let mut rng = rand::thread_rng();
    let mut object_1 = crate::object::Object::new(&mut rng, 1, 1.0, 1.0);
    object_1.cell.position.x = 0.0;
    object_1.cell.position.y = 0.0;
    object_1.cell.energy = 1;
    let mut object_2 = crate::object::Object::new(&mut rng, 2, 1.0, 1.0);
    object_2.cell.position.x = -0.1;
    object_2.cell.position.y = -0.1;
    object_2.cell.energy = 2;
    let mut object_list = [object_1, object_2];
    crate::system::sensor::process(&mut object_list);
    assert_eq!(
        object_list[0].cell.sensor.data_list,
        [-14.142135623730947, 0.0, 0.0, -14.142135623730947]
    );
}

#[test]
fn danger_ru_test() {
    let mut rng = rand::thread_rng();
    let mut object_1 = crate::object::Object::new(&mut rng, 1, 1.0, 1.0);
    object_1.cell.position.x = 0.0;
    object_1.cell.position.y = 0.0;
    object_1.cell.energy = 1;
    let mut object_2 = crate::object::Object::new(&mut rng, 2, 1.0, 1.0);
    object_2.cell.position.x = 0.1;
    object_2.cell.position.y = -0.1;
    object_2.cell.energy = 2;
    let mut object_list = [object_1, object_2];
    crate::system::sensor::process(&mut object_list);
    assert_eq!(
        object_list[0].cell.sensor.data_list,
        [-14.142135623730947, -14.142135623730947, 0.0, 0.0]
    );
}

#[test]
fn danger_ld_test() {
    let mut rng = rand::thread_rng();
    let mut object_1 = crate::object::Object::new(&mut rng, 1, 1.0, 1.0);
    object_1.cell.position.x = 0.0;
    object_1.cell.position.y = 0.0;
    object_1.cell.energy = 1;
    let mut object_2 = crate::object::Object::new(&mut rng, 2, 1.0, 1.0);
    object_2.cell.position.x = -0.3;
    object_2.cell.position.y = 0.3;
    object_2.cell.energy = 2;
    let mut object_list = [object_1, object_2];
    crate::system::sensor::process(&mut object_list);
    assert_eq!(
        object_list[0].cell.sensor.data_list,
        [0.0, 0.0, -4.714045207910317, -4.714045207910317]
    );
}

#[test]
fn food_ld_test() {
    let mut rng = rand::thread_rng();
    let mut object_1 = crate::object::Object::new(&mut rng, 1, 1.0, 1.0);
    object_1.cell.position.x = 0.0;
    object_1.cell.position.y = 0.0;
    object_1.cell.energy = 2;
    let mut object_2 = crate::object::Object::new(&mut rng, 2, 1.0, 1.0);
    object_2.cell.position.x = -0.1;
    object_2.cell.position.y = 0.1;
    object_2.cell.energy = 3;
    let mut object_3 = crate::object::Object::new(&mut rng, 3, 1.0, 1.0);
    object_3.cell.position.x = -0.01;
    object_3.cell.position.y = 0.01;
    object_3.cell.energy = 1;
    let mut object_list = [object_1, object_2, object_3];
    crate::system::sensor::process(&mut object_list);
    assert_eq!(
        object_list[0].cell.sensor.data_list,
        [0.0, 0.0, 49.49747468305833, 49.49747468305833]
    );
}

#[test]
fn equal_rd_test() {
    let mut rng = rand::thread_rng();
    let mut object_1 = crate::object::Object::new(&mut rng, 1, 1.0, 1.0);
    object_1.cell.position.x = 0.0;
    object_1.cell.position.y = 0.0;
    object_1.cell.energy = 2;
    let mut object_2 = crate::object::Object::new(&mut rng, 2, 1.0, 1.0);
    object_2.cell.position.x = -0.1;
    object_2.cell.position.y = 0.1;
    object_2.cell.energy = 2;
    let mut object_3 = crate::object::Object::new(&mut rng, 3, 1.0, 1.0);
    object_3.cell.position.x = -0.01;
    object_3.cell.position.y = 0.01;
    object_3.cell.energy = 2;
    let mut object_list = [object_1, object_2, object_3];
    crate::system::sensor::process(&mut object_list);
    assert_eq!(
        object_list[0].cell.sensor.data_list,
        [0.0, 0.0, -77.78174593052023, -77.78174593052023]
    );
    assert_eq!(
        object_list[1].cell.sensor.data_list,
        [-14.927809825049334, -14.927809825049334, 0.0, 0.0]
    );
    assert_eq!(
        object_list[2].cell.sensor.data_list,
        [
            -70.71067811865476,
            -70.71067811865476,
            -7.85674201318386,
            -7.85674201318386
        ]
    );
}
