#[cfg(target_os = "android")]
fn main() {
    println!("This example is not supported on Android (https://github.com/rust-windowing/winit/issues/948).")
}

#[cfg(all(not(target_os = "android"), not(feature = "gl"), not(feature = "shaper")))]
fn main() {
    println!("To run this example, invoke cargo with --features \"gl,shaper\".")
}

// Generated with https://generator.lorem-ipsum.info/_arabic
const LOREM_IPSUM: &str = include_str!("./Arabic-Lipsum.txt");

#[cfg(all(not(target_os = "android"), feature = "gl", feature = "shaper"))]
fn main() {
    use skia_safe::gpu::gl::FramebufferInfo;
    use skia_safe::gpu::{BackendRenderTarget, Context, SurfaceOrigin};
    use skia_safe::{Color, ColorType, Paint, Surface, Font, Typeface, FontStyle, Point, Shaper, icu, FontMgr};
    use skia_safe::font_style::{Weight, Width, Slant};
    use skia_safe::font::Edging;
    use std::convert::TryInto;

    use glutin::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
    use glutin::event_loop::{ControlFlow, EventLoop};
    use glutin::window::WindowBuilder;
    use glutin::{ContextBuilder, GlProfile};

    use gl::types::*;
    use gl_rs as gl;

    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("rust-skia-gl-window-text-blob");

    let cb = ContextBuilder::new()
        .with_depth_buffer(0)
        .with_stencil_buffer(8)
        .with_pixel_format(24, 8)
        .with_double_buffer(Some(true))
        .with_gl_profile(GlProfile::Core);

    let windowed_context = cb.build_windowed(wb, &el).unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    let pixel_format = windowed_context.get_pixel_format();

    println!(
        "Pixel format of the window's GL context: {:?}",
        pixel_format
    );

    gl::load_with(|s| windowed_context.get_proc_address(&s));

    let mut gr_context = Context::new_gl(None).unwrap();

    let mut fboid: GLint = 0;
    unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

    let fb_info = FramebufferInfo {
        fboid: fboid.try_into().unwrap(),
        format: skia_safe::gpu::gl::Format::RGBA8.into(),
    };

    let size = windowed_context.window().inner_size();
    let backend_render_target = BackendRenderTarget::new_gl(
        (
            size.width.try_into().unwrap(),
            size.height.try_into().unwrap(),
        ),
        pixel_format.multisampling.map(|s| s.try_into().unwrap()),
        pixel_format.stencil_bits.try_into().unwrap(),
        fb_info,
    );
    let mut surface = Surface::from_backend_render_target(
        &mut gr_context,
        &backend_render_target,
        SurfaceOrigin::BottomLeft,
        ColorType::RGBA8888,
        None,
        None,
    )
    .unwrap();

    let font_style = FontStyle::new(Weight::NORMAL, Width::NORMAL, Slant::Upright);
    let typeface = Typeface::new("Arial", font_style).unwrap();
    let mut font = Font::from_typeface(typeface, 16.0);
    let shaper = Shaper::new_shape_then_wrap(Some(FontMgr::default())).unwrap();

    font.set_edging(Edging::SubpixelAntiAlias);
    font.set_subpixel(true);

    icu::init();

    let sf = windowed_context.window().scale_factor() as f32;
    surface.canvas().scale((sf, sf));

    let mut x = 0;

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        #[allow(deprecated)]
        match event {
            Event::LoopDestroyed => {}
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode,
                            modifiers,
                            ..
                        },
                    ..
                } => {
                    if modifiers.logo() {
                        if let Some(VirtualKeyCode::Q) = virtual_keycode {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                    x += 1;
                    windowed_context.window().request_redraw();
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                {
                    let canvas = surface.canvas();
                    let paint = Paint::default();

                    canvas.clear(Color::WHITE);

                    let (text_blob, _) = shaper.shape_text_blob(LOREM_IPSUM, &font, true, 500.0, Point::default()).expect("Could not shape text");
                    canvas.draw_text_blob(&text_blob, Point::new(32.0, 32.0), &paint);
                }
                surface.canvas().flush();
                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
