use conquest_tiles::tilemap::{NavPoint, TileDefinition, TileMap};

fn main() {
    let mut tm = TileMap::new(24, 12);

    tm.sprites.push(TileDefinition {
        path: "floor.png".into(),
        cost: 1,
    });
    tm.sprites.push(TileDefinition {
        path: "wall.png".into(),
        cost: i8::MAX,
    });

    tm.set(5, 5, 1);
    tm.set(9, 9, 1);

    tm.nav_points.push(NavPoint {
        id: 0,
        x: 1,
        y: 1,
        edges: vec![],
    });

    tm.nav_points.push(NavPoint {
        id: 1,
        x: 5,
        y: 5,
        edges: vec![0],
    });

    let tm_str = tm.to_string();
    println!("STRING {}", tm_str);

    let parsed_tm = TileMap::from_str(tm_str.as_str());
    println!("PARSED {:?}", parsed_tm);
}
