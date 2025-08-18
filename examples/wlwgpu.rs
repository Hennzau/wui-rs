use wui_rs::*;

fn main() -> Result<()> {
    let mut wlwgpu = wlwgpu()?;

    pollster::block_on(SurfaceBuilder::default().window(true).build(&mut wlwgpu))?;

    let mut scene = Scene::new();
    let mut x = 0.0;

    wlwgpu.run(move |shell, event| {
        let id = event.id;
        let kind = event.kind;

        match kind {
            EventKind::Close => {
                if let Some(id) = id {
                    shell.destroy_surface(&id);
                }

                if shell.surfaces() < 1 {
                    shell.stop();
                }
            }
            EventKind::Resize { width, height } => {
                if let Some(id) = id {
                    shell.resize_surface(&id, width, height);
                }
            }
            EventKind::Draw => {
                if let Some(id) = id {
                    scene.clear();

                    let (width, height) = shell.size(&id).unwrap();

                    scene.fill(width, height, Color::WHITE);
                    scene.add_circle(x, (height / 2) as f64, height as f64 / 7.0, Color::BLACK);

                    if let Err(e) = shell.render(&id, &scene) {
                        println!("Error: {}", e);
                    }

                    x += 1000.0 * 6.0 / 1000.0;

                    shell.request_redraw(&id);
                }
            }
            _ => {}
        }

        Ok(())
    })
}
