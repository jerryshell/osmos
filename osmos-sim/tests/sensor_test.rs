#[test]
fn danger_u_test() {
    let mut rng = rand::thread_rng();
    let mut object_1 = osmos_sim::object::Object::new(&mut rng, 1);
    object_1.cell.position.x = 0.0;
    object_1.cell.position.y = 0.0;
    object_1.cell.energy = 1;
    let mut object_2 = osmos_sim::object::Object::new(&mut rng, 2);
    object_2.cell.position.x = 0.0;
    object_2.cell.position.y = 0.1;
    object_2.cell.energy = 2;
    let mut object_list = [object_1, object_2];
    osmos_sim::system::sensor::process(&mut object_list);
    assert_eq!(object_list[0].cell.sensor.data_list, [0.0, -1.0, 0.0, 0.0]);
    assert_eq!(object_list[1].cell.sensor.data_list, [0.0, 0.0, 0.0, 1.0]);
}

#[test]
fn danger_r_test() {
    let mut rng = rand::thread_rng();
    let mut object_1 = osmos_sim::object::Object::new(&mut rng, 1);
    object_1.cell.position.x = 0.0;
    object_1.cell.position.y = 0.0;
    object_1.cell.energy = 1;
    let mut object_2 = osmos_sim::object::Object::new(&mut rng, 2);
    object_2.cell.position.x = 0.1;
    object_2.cell.position.y = 0.0;
    object_2.cell.energy = 2;
    let mut object_list = [object_1, object_2];
    osmos_sim::system::sensor::process(&mut object_list);
    assert_eq!(object_list[0].cell.sensor.data_list, [-1.0, 0.0, 0.0, 0.0]);
    assert_eq!(object_list[1].cell.sensor.data_list, [0.0, 0.0, 1.0, 0.0]);
}

#[test]
fn danger_d_test() {
    let mut rng = rand::thread_rng();
    let mut object_1 = osmos_sim::object::Object::new(&mut rng, 1);
    object_1.cell.position.x = 0.0;
    object_1.cell.position.y = 0.0;
    object_1.cell.energy = 1;
    let mut object_2 = osmos_sim::object::Object::new(&mut rng, 2);
    object_2.cell.position.x = 0.0;
    object_2.cell.position.y = -0.1;
    object_2.cell.energy = 2;
    let mut object_list = [object_1, object_2];
    osmos_sim::system::sensor::process(&mut object_list);
    assert_eq!(object_list[0].cell.sensor.data_list, [0.0, 0.0, 0.0, -1.0]);
    assert_eq!(object_list[1].cell.sensor.data_list, [0.0, 1.0, 0.0, 0.0]);
}

#[test]
fn danger_l_test() {
    let mut rng = rand::thread_rng();
    let mut object_1 = osmos_sim::object::Object::new(&mut rng, 1);
    object_1.cell.position.x = 0.0;
    object_1.cell.position.y = 0.0;
    object_1.cell.energy = 1;
    let mut object_2 = osmos_sim::object::Object::new(&mut rng, 2);
    object_2.cell.position.x = -0.1;
    object_2.cell.position.y = 0.0;
    object_2.cell.energy = 2;
    let mut object_list = [object_1, object_2];
    osmos_sim::system::sensor::process(&mut object_list);
    assert_eq!(object_list[0].cell.sensor.data_list, [0.0, 0.0, -1.0, 0.0]);
    assert_eq!(object_list[1].cell.sensor.data_list, [1.0, 0.0, 0.0, 0.0]);
}

#[test]
fn danger_rd_test() {
    let mut rng = rand::thread_rng();
    let mut object_1 = osmos_sim::object::Object::new(&mut rng, 1);
    object_1.cell.position.x = 0.0;
    object_1.cell.position.y = 0.0;
    object_1.cell.energy = 1;
    let mut object_2 = osmos_sim::object::Object::new(&mut rng, 2);
    object_2.cell.position.x = 0.1;
    object_2.cell.position.y = 0.1;
    object_2.cell.energy = 2;
    let mut object_list = [object_1, object_2];
    osmos_sim::system::sensor::process(&mut object_list);
    assert_eq!(object_list[0].cell.sensor.data_list, [-1.0, 0.0, 0.0, 0.0]);
    assert_eq!(object_list[1].cell.sensor.data_list, [0.0, 0.0, 1.0, 0.0]);
}

#[test]
fn danger_lu_test() {
    assert_eq!(
        nalgebra::Vector2::new(-1.0, -1.0).angle(&nalgebra::Vector2::new(1.0, 0.0)),
        nalgebra::Vector2::new(-1.0, 1.0).angle(&nalgebra::Vector2::new(1.0, 0.0))
    );
    let mut rng = rand::thread_rng();
    let mut object_1 = osmos_sim::object::Object::new(&mut rng, 1);
    object_1.cell.position.x = 0.0;
    object_1.cell.position.y = 0.0;
    object_1.cell.energy = 1;
    let mut object_2 = osmos_sim::object::Object::new(&mut rng, 2);
    object_2.cell.position.x = -0.1;
    object_2.cell.position.y = -0.1;
    object_2.cell.energy = 2;
    let mut object_list = [object_1, object_2];
    osmos_sim::system::sensor::process(&mut object_list);
    assert_eq!(object_list[0].cell.sensor.data_list, [0.0, 0.0, -1.0, 0.0]);
}

#[test]
fn danger_ru_test() {
    let mut rng = rand::thread_rng();
    let mut object_1 = osmos_sim::object::Object::new(&mut rng, 1);
    object_1.cell.position.x = 0.0;
    object_1.cell.position.y = 0.0;
    object_1.cell.energy = 1;
    let mut object_2 = osmos_sim::object::Object::new(&mut rng, 2);
    object_2.cell.position.x = 0.1;
    object_2.cell.position.y = -0.1;
    object_2.cell.energy = 2;
    let mut object_list = [object_1, object_2];
    osmos_sim::system::sensor::process(&mut object_list);
    assert_eq!(object_list[0].cell.energy, 1);
    assert_eq!(object_list[1].cell.energy, 2);
    assert_eq!(object_list[0].cell.sensor.data_list, [0.0, 0.0, 0.0, -1.0]);
}

#[test]
fn danger_ld_test() {
    let mut rng = rand::thread_rng();
    let mut object_1 = osmos_sim::object::Object::new(&mut rng, 1);
    object_1.cell.position.x = 0.0;
    object_1.cell.position.y = 0.0;
    object_1.cell.energy = 1;
    let mut object_2 = osmos_sim::object::Object::new(&mut rng, 2);
    object_2.cell.position.x = -0.3;
    object_2.cell.position.y = 0.3;
    object_2.cell.energy = 2;
    let mut object_list = [object_1, object_2];
    osmos_sim::system::sensor::process(&mut object_list);
    assert_eq!(object_list[0].cell.sensor.data_list, [0.0, -1.0, 0.0, 0.0]);
}

#[test]
fn food_ld_test() {
    let mut rng = rand::thread_rng();
    let mut object_1 = osmos_sim::object::Object::new(&mut rng, 1);
    object_1.cell.position.x = 0.0;
    object_1.cell.position.y = 0.0;
    object_1.cell.energy = 2;
    let mut object_2 = osmos_sim::object::Object::new(&mut rng, 2);
    object_2.cell.position.x = -0.1;
    object_2.cell.position.y = 0.1;
    object_2.cell.energy = 3;
    let mut object_3 = osmos_sim::object::Object::new(&mut rng, 3);
    object_3.cell.position.x = -0.01;
    object_3.cell.position.y = 0.01;
    object_3.cell.energy = 1;
    let mut object_list = [object_1, object_2, object_3];
    osmos_sim::system::sensor::process(&mut object_list);
    assert_eq!(object_list[0].cell.sensor.data_list, [0.0, 1.0, 0.0, 0.0]);
}

#[test]
fn equal_rd_test() {
    let mut rng = rand::thread_rng();
    let mut object_1 = osmos_sim::object::Object::new(&mut rng, 1);
    object_1.cell.position.x = 0.0;
    object_1.cell.position.y = 0.0;
    object_1.cell.energy = 2;
    let mut object_2 = osmos_sim::object::Object::new(&mut rng, 2);
    object_2.cell.position.x = -0.1;
    object_2.cell.position.y = 0.1;
    object_2.cell.energy = 2;
    let mut object_3 = osmos_sim::object::Object::new(&mut rng, 3);
    object_3.cell.position.x = -0.01;
    object_3.cell.position.y = 0.01;
    object_3.cell.energy = 2;
    let mut object_list = [object_1, object_2, object_3];
    osmos_sim::system::sensor::process(&mut object_list);
    assert_eq!(object_list[0].cell.sensor.data_list, [0.0, 0.0, 0.0, 0.0]);
    assert_eq!(object_list[1].cell.sensor.data_list, [0.0, 0.0, 0.0, 0.0]);
    assert_eq!(object_list[2].cell.sensor.data_list, [0.0, 0.0, 0.0, 0.0]);
}
