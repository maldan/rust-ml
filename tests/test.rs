mod tests {
    use ml::io::mouse;
    use ml::math::la::quaternion::Quaternion;
    use ml::math::la::vector2::Vector2;
    use ml::math::la::vector3::Vector3;
    use ml::math::random::Random;
    use ml::{math, to_rad};

    #[test]
    fn quat() {
        let q1 = Quaternion::from_euler(Vector3::new(45.0f32, 0.0, 0.0).to_radians());

        let q2 = Quaternion::from_euler(Vector3::new(45.0f32, 0.0, 0.0).to_radians());
        println!("{}", q1 * q2);

        let v1 = (q1 * q2).to_euler().to_degrees();
        println!("{}", v1);

        // wasm_logger::init(wasm_logger::Config::default());
        // log::info!("Some info");
    }

    #[test]
    fn sas() {
        let v1 = Vector3::default();
        let v2 = Vector3::new(1.0, 1.0, 1.0);
        println!("{}", (v1 - v2).normalize());
    }

    #[test]
    fn sas2() {
        let v1 = Vector2::default();
        let v2 = Vector2::new(1.0, 1.0);
        println!("{}", (v1 - v2).normalize());
    }

    #[test]
    fn sas3() {
        println!("{}", to_rad!(180));
    }

    #[test]
    fn sas4() {
        let mut r = Random::new(0);

        for _i in 0..100 {
            println!("{}", r.range(-4.0, 4.0));
        }
    }

    #[test]
    fn sas5() {
        println!("{}", math::lerp(0.0, 1.0, 0.5));
    }

    #[test]
    fn remap() {
        println!("{}", math::remap(1.0, 0.0, 1.0, 0.0, 5.0));
    }

    #[test]
    fn mouse() {
        println!("{}", mouse::is_down(0));
        println!("{}", mouse::is_down(12222));
        println!("{}", mouse::is_down(1000));
    }
}
