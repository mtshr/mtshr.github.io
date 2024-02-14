mod vec3;

use std::{array, f64::consts::TAU};

use vec3::Vec3;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

use wasm_bindgen::prelude::*;

enum Msg {
	Tick,
}

struct AnimationCanvas {
	canvas: NodeRef,
	time: u32,
	angle: [f64; 3],
	angle_velocity: [f64; 3],
	vertices: Vec<Vec3>,
	callback: Closure<dyn FnMut()>,
}

fn cube() -> Vec<Vec3> {
	let mut v = Vec::with_capacity(5 * 5 * 5);
	for x in -2..=2 {
		for y in -2..=2 {
			for z in -2..=2 {
				v.push(Vec3 {
					x: x as f64,
					y: y as f64,
					z: z as f64,
				})
			}
		}
	}
	v
}

impl Component for AnimationCanvas {
	type Message = Msg;
	type Properties = ();

	fn create(ctx: &Context<Self>) -> Self {
		let ctx = ctx.link().clone();

		Self {
			canvas: NodeRef::default(),
			time: 0,
			angle: array::from_fn(|_| rand::random::<f64>() * TAU),
			angle_velocity: [0.01, 0.02, 0.03],
			vertices: cube(),
			callback: Closure::wrap(Box::new(move || ctx.send_message(Msg::Tick))),
		}
	}

	fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
		if first_render {
			ctx.link().send_message(Msg::Tick);
		}
	}

	fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
			Msg::Tick => {
				self.tick();
				self.render();
				window()
					.unwrap()
					.request_animation_frame(self.callback.as_ref().unchecked_ref())
					.unwrap();
				false
			}
		}
	}

	fn view(&self, _ctx: &Context<Self>) -> Html {
		html! {
			<canvas id="canvas" width={640} height={640} ref={self.canvas.clone()}></canvas>
		}
	}
}

impl AnimationCanvas {
	fn tick(&mut self) {
		self.time = self.time.wrapping_add(1);

		// update angle
		for (a, v) in self.angle.iter_mut().zip(self.angle_velocity.iter_mut()) {
			let delta = rand::random::<f64>() - 0.5;
			*v = (*v + delta / 500.).clamp(0.01, 0.04);
			*a = (*a + *v) % TAU;
		}
	}

	fn render(&self) {
		let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
		let ctx: CanvasRenderingContext2d =
			canvas.get_context("2d").unwrap().unwrap().unchecked_into();

		let width = canvas.width() as f64;
		let height = canvas.height() as f64;
		let center = Vec3 {
			x: width / 2.,
			y: height / 2.,
			z: 0f64,
		};

		ctx.set_fill_style(&"#000".into());
		ctx.fill_rect(0f64, 0f64, width, height);

		let mat = {
			let (s1, c1) = self.angle[0].sin_cos();
			let (s2, c2) = self.angle[1].sin_cos();
			let (s3, c3) = self.angle[2].sin_cos();
			[
				[c2, -c3 * s2, s2 * s3],
				[c1 * s2, c1 * c2 * c3 - s1 * s3, -c3 * s1 - c1 * c2 * s3],
				[s1 * s2, c1 * s3 + c2 * c3 * s1, c1 * c3 - c2 * s1 * s3],
			]
		};

		let mut vertices: Vec<_> = self
			.vertices
			.iter()
			.map(|Vec3 { x, y, z }| Vec3 {
				x: mat[0][0] * x + mat[0][1] * y + mat[0][2] * z,
				y: mat[1][0] * x + mat[1][1] * y + mat[1][2] * z,
				z: mat[2][0] * x + mat[2][1] * y + mat[2][2] * z,
			})
			.enumerate()
			.collect();
		vertices.sort_by(|(_, a), (_, b)| a.z.partial_cmp(&b.z).unwrap());

		for (i, Vec3 { x, y, z }) in vertices.into_iter() {
			let t = (self.time & 255) as f64 / 256. * TAU + (123454321 % (i * i + 1)) as f64;
			let (dx, dy) = {
				let a = (i * i % 5) as f64;
				let b = (i % 3) as f64;
				((a * t).cos(), (b * t).cos())
			};

			let gray_scale = (192. + (x / 2. - y + z) * 32.).clamp(0f64, 255.) as u8;
			let co = z * 1.5 + 60.;
			let x = center.x + (x + 0.03 * dx) * co;
			let y = center.y + (y + 0.03 * dy) * co;
			let r = 8.3 + 1.25 * t.cos() + z / 2.5;
			ctx.set_fill_style(&format!("#{x:02X}{x:02X}{x:02X}", x = gray_scale).into());
			ctx.begin_path();
			ctx.arc(x, y, r, 0f64, TAU).unwrap();
			ctx.fill();
		}
	}
}

#[function_component(App)]
fn app() -> Html {
	html! {
		<div style={"text-align: center"}>
			<AnimationCanvas />
			<div>
				<a href={"https://github.com/mtshr"}>{"@mtshr"}</a>
			</div>
		</div>
	}
}

#[wasm_bindgen]
pub fn run_app() {
	yew::Renderer::<App>::new().render();
}
