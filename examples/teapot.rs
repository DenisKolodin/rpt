use rpt::*;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut scene = Scene::new();

    scene.add(
        Object::new(
            load_obj("examples/teapot.obj")?
                .scale(&glm::vec3(0.5, 0.5, 0.5))
                .translate(&glm::vec3(0.0, -1.0, 0.0)),
        )
        .material(Material::metallic(hex_color(0xff0000), 0.4)),
    );
    scene.add(
        Object::new(plane(glm::vec3(0.0, 1.0, 0.0), -1.0))
            .material(Material::diffuse(hex_color(0xaaaaaa))),
    );

    scene.add(Light::Ambient(glm::vec3(0.02, 0.02, 0.02)));
    scene.add(Light::Point(
        glm::vec3(36.0, 36.0, 36.0),
        glm::vec3(0.0, 5.0, 5.0),
    ));

    // black background
    scene.background = hex_color(0x000000);

    Renderer::new(&scene, Camera::default())
        .width(800)
        .height(800)
        .render()
        .save("output.png")?;

    Ok(())
}
