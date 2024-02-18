#![windows_subsystem = "windows"]
#![warn(clippy::pedantic, elided_lifetimes_in_paths, explicit_outlives_requirements)]
#![allow(
	confusable_idents,
	mixed_script_confusables,
	non_camel_case_types,
	non_snake_case,
	uncommon_codepoints
)]

use {
	game_animation_programming::{
		sdl_main::SdlMain, Application, Texture2d_Ext, FRAGMENT_SHADER, SAMPLER_BEHAVIOR, VERTEX_SHADER,
	},
	glam::{Mat4, Quat, Vec3},
	glium::{
		implement_vertex, index::PrimitiveType::TrianglesList, uniform, uniforms::Sampler, DrawParameters,
		IndexBuffer, Program, Surface, Texture2d, VertexBuffer,
	},
	glium_sdl2::SDL2Facade,
};

#[derive(Copy, Clone)]
struct Position {
	a_position: [f32; 3],
}
implement_vertex!(Position, a_position);

#[derive(Copy, Clone)]
struct Normal {
	a_normal: [f32; 3],
}
implement_vertex!(Normal, a_normal);

#[derive(Copy, Clone)]
struct TexCoord {
	a_texCoord: [f32; 2],
}
implement_vertex!(TexCoord, a_texCoord);

struct Sample {
	rotation: f32,
	glProgram: Program,
	positions: VertexBuffer<Position>,
	normals: VertexBuffer<Normal>,
	texCoords: VertexBuffer<TexCoord>,
	indices: IndexBuffer<u32>,
	displayTexture: Texture2d,
}

impl Sample {
	fn new(display: &SDL2Facade) -> Self {
		Self {
			rotation: 0.,
			glProgram: Program::from_source(display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap(),
			positions: VertexBuffer::new(
				display,
				&[
					Position { a_position: [-1., -1., 0.] },
					Position { a_position: [-1., 1., 0.] },
					Position { a_position: [1., -1., 0.] },
					Position { a_position: [1., 1., 0.] },
				],
			)
			.unwrap(),
			normals: VertexBuffer::new(display, &[Normal { a_normal: [0., 0., 1.] }; 4]).unwrap(),
			texCoords: VertexBuffer::new(
				display,
				&[
					TexCoord { a_texCoord: [0., 0.] },
					TexCoord { a_texCoord: [0., 1.] },
					TexCoord { a_texCoord: [1., 0.] },
					TexCoord { a_texCoord: [1., 1.] },
				],
			)
			.unwrap(),
			indices: IndexBuffer::new(display, TrianglesList, &[0, 1, 2, 2, 1, 3]).unwrap(),
			displayTexture: Texture2d::fromImageFilePath(display, "assets/uv.png"),
		}
	}
}

impl Application for Sample {
	fn update(&mut self, deltaSeconds: f32) {
		let o = self;
		o.rotation += deltaSeconds * 45.;
		while o.rotation > 360. {
			o.rotation -= 360.;
		}
	}

	fn render(&self, frame: &mut impl Surface, drawParameters: &DrawParameters<'_>, aspectRatio: f32) {
		let o = self;
		frame
			.draw(
				(&o.positions, &o.normals, &o.texCoords),
				&o.indices,
				&o.glProgram,
				&uniform! {
					u_model:
						Mat4::from_quat(Quat::from_axis_angle(Vec3::Z, o.rotation.to_radians())).to_cols_array_2d(),
					u_view: Mat4::look_at_rh(Vec3::new(0., 0., -5.), Vec3::ZERO, Vec3::Y).to_cols_array_2d(),
					u_projection:
						Mat4::perspective_rh_gl((60_f32).to_radians(), aspectRatio, 0.01, 1000.).to_cols_array_2d(),
					u_light: (Vec3::Z).to_array(),
					u_texture: Sampler(&o.displayTexture, SAMPLER_BEHAVIOR),
				},
				drawParameters,
			)
			.unwrap();
	}
}

fn main() {
	let sdlMain = &mut SdlMain::new(file!());
	let sample = &mut Sample::new(&sdlMain.display);
	sdlMain.run(sample);
}
