// TODO: Fix this example when we switch to actively maintained rendering
// backend. Right now we would have to update `gfx_window_glutin` to work with
// the latest version of glutin or we would need to add headless support to
// `old_school_gfx_glutin_ext`.

fn main() {
    println!("Example temporarily disabled, see the TODO comment for details");
}
/*use common::comp;
use gfx_window_glutin::init_headless;
use vek::*;
use veloren_voxygen::{render, scene::simple as scene};

fn main() {
    // Setup renderer
    let dim = (200u16, 300u16, 1, gfx::texture::AaMode::Single);
    let events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 2)))
        .build_headless(&events_loop, (dim.0 as u32, dim.1 as u32).into())
        .expect("Failed to build headless context");

    let (_context, device, factory, color_view, depth_view) = init_headless(context, dim);

    let mut renderer =
        render::Renderer::new(device, factory, color_view, depth_view, Default::default()).unwrap();

    // Create character
    let body = comp::humanoid::Body::random();

    let loadout = comp::Loadout {
        active_item: None,
        second_item: None,
        shoulder: None,
        chest: None,
        belt: None,
        hand: None,
        pants: None,
        foot: None,
        back: None,
        ring: None,
        neck: None,
        lantern: None,
        head: None,
        tabard: None,
    };

    // Setup scene (using the character selection screen `Scene`)
    let mut scene = scene::Scene::new(&mut renderer, None);
    let scene_data = scene::SceneData {
        time: 1.0,
        delta_time: 1.0,
        tick: 0,
        body: Some(body.clone()),
        gamma: 1.0,
        mouse_smoothing: true,
        figure_lod_render_distance: 250.0,
    };
    scene.camera_mut().set_focus_pos(Vec3::unit_z() * 0.8);
    scene.camera_mut().set_distance(1.5);
    scene
        .camera_mut()
        .update(0.0, 1.0 / 60.0, scene_data.mouse_smoothing);
    scene.maintain(&mut renderer, scene_data, Some(&loadout));

    // Render
    renderer.clear();
    scene.render(&mut renderer, 0, Some(body), Some(&loadout));

    renderer.flush();
    // Get image
    let img = renderer.create_screenshot().unwrap();
    img.save("character.png").unwrap();
}*/
