use {
	crate::{default, Application},
	glium::{BackfaceCullingMode::CullClockwise, Depth, DepthTest::IfLess, DrawParameters, Surface},
	glium_sdl2::{DisplayBuild, SDL2Facade},
	sdl2::{event::Event, keyboard::Scancode, Sdl},
	std::{
		env, thread,
		time::{Duration, Instant},
	},
};

pub struct SdlMain {
	sdl2: Sdl,
	pub display: SDL2Facade,
}

impl SdlMain {
	pub fn new(app_srcFilePath: &str) -> Self {
		if cfg!(windows) {
			env::set_current_dir(env::current_exe().unwrap().parent().unwrap()).unwrap();
		}
		let sdl2 = sdl2::init().unwrap();
		let display = {
			let video = sdl2.video().unwrap();
			{
				let glAttr = video.gl_attr();
				// glAttr.set_context_profile(video::GLProfile::GLES);
				glAttr.set_context_version(2, 0);
			}
			video.window(app_srcFilePath, 800, 600).resizable().build_glium().unwrap()
		};
		Self { sdl2, display }
	}

	pub fn run(&self, app: &mut impl Application) {
		const FPS: u32 = 30;
		let (o, frameDuration) = (self, Duration::from_secs(1) / FPS);
		let (mut eventPump, mut nextFrameInstant) =
			(o.sdl2.event_pump().unwrap(), Instant::now() + frameDuration);
		loop {
			for event in eventPump.poll_iter() {
				match event {
					Event::Quit { .. } | Event::KeyDown { scancode: Some(Scancode::Escape), .. } => return,
					_ => {}
				}
			}
			app.update(frameDuration.as_secs_f32());
			{
				let mut frame = o.display.draw();
				frame.clear_color_srgb_and_depth((0.5, 0.6, 0.7, 1.0), 1.0);
				app.render(
					&mut frame,
					&DrawParameters {
						depth: Depth { test: IfLess, write: true, ..default() },
						backface_culling: CullClockwise,
						..default()
					},
					{
						let (width, height) = o.display.get_framebuffer_dimensions();
						(width as f32) / (height as f32)
					},
				);
				frame.finish().unwrap();
			}
			thread::sleep(nextFrameInstant - Instant::now());
			nextFrameInstant += frameDuration;
		}
	}
}
