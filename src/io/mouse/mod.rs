#[allow(dead_code)]
static CLICK_STATE: [bool; 32] = [false; 32];

#[allow(dead_code)]
static STATE: [bool; 32] = [false; 32];

#[allow(dead_code)]
static POSITION: [f32; 2] = [0.0, 0.0];

#[allow(dead_code)]
enum Button {
    Left = 0,
    Right = 1,
    Middle = 2,
}

pub fn is_down(key: usize) -> bool {
    match STATE.get(key) {
        Some(bool) => *bool,
        _ => false,
    }
}

pub fn get_position() -> (f32, f32) {
    return (POSITION[0], POSITION[1]);
}
