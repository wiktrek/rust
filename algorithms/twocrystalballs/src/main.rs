fn main() {
    let a = [
        false, false, false, false, false, false, false, false, true, true, true, true, true, true,
        true, true, true, true,
    ];
    let jump_amout = (a.len() as f32).sqrt().floor();
    let mut i = jump_amout;
    let mut j = 0;
    while i <= a.len() as f32 {
        if a[i as usize] {
            break;
        }
        i += jump_amout;
    }
    i -= jump_amout;
    while j as f32 <= jump_amout {
        if a[i as usize] {
            return println!("ez {i}");
        }
        j += 1;
        i += 1.0;
    }
}
