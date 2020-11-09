// This example demonstrates rendering with a kd-tree of kd-trees.

use rpt::*;
use std::sync::Arc;

fn gen(
    obj: Arc<Mesh>,
    spheres: &mut [Vec<Box<dyn Bounded>>],
    x: f64,
    y: f64,
    z: f64,
    rad: f64,
    depth: usize,
    last_dir: Option<usize>,
) {
    spheres[depth].push(Box::new(
        Arc::clone(&obj)
            .scale(&glm::vec3(0.5, 0.5, 0.5))
            .scale(&glm::vec3(rad, rad, rad))
            .translate(&glm::vec3(x, y, z)),
    ));
    if depth == spheres.len() - 1 {
        return;
    }
    let disp = rad * 7.0 / 5.0;
    let dx: [f64; 6] = [disp, -disp, 0.0, 0.0, 0.0, 0.0];
    let dy: [f64; 6] = [0.0, 0.0, disp, -disp, 0.0, 0.0];
    let dz: [f64; 6] = [0.0, 0.0, 0.0, 0.0, disp, -disp];
    for i in 0..6 {
        if last_dir.is_none() || i != (last_dir.unwrap() ^ 1) {
            gen(
                Arc::clone(&obj),
                spheres,
                x + dx[i],
                y + dy[i],
                z + dz[i],
                rad * 2.0 / 5.0,
                depth + 1,
                Some(i),
            );
        }
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let colors = [0x264653, 0x2A9D8F, 0xE9C46A, 0xF4A261, 0xE76F51];
    let mut spheres: Vec<_> = colors.iter().map(|_| Vec::new()).collect();

    let teapot = Arc::new(load_obj("examples/teapot.obj")?);
    gen(teapot, &mut spheres, 0.0, 0.0, 0.0, 1.0, 0, None);

    let mut scene = Scene::new();
    for (i, sphere_group) in spheres.into_iter().enumerate() {
        println!("Level {}: {} spheres", i, sphere_group.len());
        scene.add(
            Object::new(KdTree::new(sphere_group))
                .material(Material::specular(hex_color(colors[i]), 0.15)),
        );
    }
    scene.add(
        Object::new(plane(glm::vec3(0.0, 0.0, 1.0), -6.0))
            .material(Material::diffuse(hex_color(0xffcccc))),
    );

    // black background
    scene.background = hex_color(0x000000);

    scene.add(Light::Ambient(glm::vec3(0.02, 0.02, 0.02)));
    scene.add(Light::Directional(
        glm::vec3(0.2, 0.2, 0.2),
        glm::vec3(0.0, -0.65, -1.0).normalize(),
    ));
    scene.add(Light::Point(
        glm::vec3(36.0, 36.0, 36.0),
        glm::vec3(0.0, 5.0, 5.0),
    ));

    let camera = Camera {
        center: glm::vec3(2.0, 3.5, 7.0),
        direction: glm::vec3(-0.285714, -0.5, -1.0).normalize(),
        up: glm::vec3(0.0, 1.0, -0.5).normalize(),
        fov: std::f64::consts::FRAC_PI_6,
    };
    Renderer::new(&scene, camera)
        .width(800)
        .height(600)
        .render()
        .save("output.png")?;

    Ok(())
}
