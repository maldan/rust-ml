#[macro_export]
macro_rules! to_rad {
    ( $e:expr ) => {
        paste::item! {
            ($e as f32) / 180.0 * 3.1415
        }
    };
}
