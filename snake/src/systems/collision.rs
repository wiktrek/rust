fn snake_collision(
    mut set: ParamSet<(
        Query<(&mut Score ,&mut Transform, Entity), With<SnakeHead>>,
        Query<(Entity, &mut Transform), With<SnakeTail>>,
    )>, 
    snake_parts: ResMut<SnakeParts>,
    mut commands: Commands
) {
    let head_position = {
        let p0 = set.p0();
        let head = p0.get_single().unwrap().1;
        Vec2::new(head.translation.x, head.translation.y)
    }; 
    let mut lost = false;
    if head_position.x > 510. || head_position.x < -510. || head_position.y > 310. || head_position.y < -310. {
    lost = true
    } else {
        // println!("x:{} y: {}", head_position.x, head_position.y)
    }
    for (_e, tail) in set.p1().iter() {
        if ((((tail.translation.x - head_position.x) as f32).powf(2.0) + ((tail.translation.y - head_position.y) as f32).powf(2.0)) as f32).sqrt() < (PLAYER / 2.0 + PLAYER / 2.0) {
            lost = true
        }
    }
    
    if lost == true {
        restart(&mut commands, set, snake_parts)
    }
}
fn restart(commands: &mut Commands,mut set: ParamSet<(
        Query<(&mut Score ,&mut Transform, Entity), With<SnakeHead>>,
        Query<(Entity, &mut Transform), With<SnakeTail>>,
    )>, mut snake_parts: ResMut<SnakeParts>) {
        snake_parts.0.clear();
        snake_parts.0.push(set.p0().get_single().unwrap().2);
    for tail in set.p1().iter_mut() {
        commands.get_entity(tail.0).unwrap().despawn();
    }
    set.p0().get_single_mut().unwrap().0.value = 0;
    set.p0().get_single_mut().unwrap().1.translation.x = 0.;
    set.p0().get_single_mut().unwrap().1.translation.y = 0.;
}