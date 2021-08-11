pub enum Direction {
    Polar {
        degree_100: i32,
    },
    Cartesian {
        x: i32,
        y: i32,
        z: i32,
    },
    Spherical {
        z_degree_100: i32,
        elevation_degree_100: i32,
    },
}
