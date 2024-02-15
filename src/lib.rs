mod vec3;

use std::{array, cell::RefCell, f64::consts::TAU};

use vec3::Vec3;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

use wasm_bindgen::prelude::*;

enum Msg {
	Tick,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
	PopIn { timer: u32 },
	Stable { timer: u32 },
	PopOut { timer: u32 },
}

impl State {
	const STABLE_DURATION: u32 = 5 * 60;
}

struct AnimationCanvas {
	canvas: NodeRef,
	time: u32,
	state: State,
	angle: [f64; 3],
	angle_velocity: [f64; 3],
	vertices: Vec<Vec3>,
	callback: Closure<dyn FnMut()>,
}

thread_local! {
	static CHOSEN_SHAPE: RefCell<usize> = const { RefCell::new(usize::MAX) };
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

	if rand::random::<f64>() > 0.5 {
		v.sort_by(|a, b| {
			a.manhattan_norm()
				.partial_cmp(&b.manhattan_norm())
				.unwrap()
				.then(a.z.partial_cmp(&b.z).unwrap())
				.then(a.y.atan2(a.x).partial_cmp(&b.y.atan2(b.x)).unwrap())
		});
	}

	v
}

fn polygon_tower() -> Vec<Vec3> {
	let mut v = Vec::with_capacity(16 * 17 / 2);

	for h in 1..=16 {
		let r = (h - 1) as f64 / 6.;
		for n in 0..h {
			let theta = n as f64 / h as f64 * TAU;
			v.push(Vec3 {
				x: r * theta.cos(),
				y: r * theta.sin(),
				z: (h - 10) as f64 / 2.5,
			})
		}
	}

	v
}

fn torus() -> Vec<Vec3> {
	fn get(n: i32, k: i32) -> Vec3 {
		const R: f64 = 1.;
		let (s, c) = ((n as f64 + k as f64 / 8.) / 16. * TAU).sin_cos();
		let theta = (k as f64 / 8. + n as f64 / 16.) * TAU;
		let x = 2.75 + R * theta.cos();
		let y = R * theta.sin();
		Vec3 {
			x: x * c,
			y,
			z: x * s,
		}
	}

	let mut v = Vec::with_capacity(8 * 16);

	if rand::random::<f64>() > 0.5 {
		for n in 0..16 {
			for k in 0..8 {
				v.push(get(n, k));
			}
		}
	} else {
		for k in 0..8 {
			for n in 0..16 {
				v.push(get(n, k));
			}
		}
	}

	v
}

fn sierpinski_gasket() -> Vec<Vec3> {
	let mut v = Vec::with_capacity(3 * 3 * 3 * 3);

	let a: Vec3 = (-0.5, -3f64.sqrt() / 2., 0f64).into();
	let b: Vec3 = (1f64, 0f64, 0f64).into();
	let c: Vec3 = (0f64, 16. * 3f64.sqrt() / 3., 0f64).into();

	for n in 0..16 {
		for k in 0..=n {
			if n & k == k {
				v.push(a * n as f64 * 0.5 + b * k as f64 * 0.5 + c * 0.5);
			}
		}
	}

	v
}

fn get_random_vertices() -> Vec<Vec3> {
	const SHAPES: [fn() -> Vec<Vec3>; 4] = [cube, polygon_tower, torus, sierpinski_gasket];
	let id = CHOSEN_SHAPE.with_borrow_mut(|id| {
		*id = if *id == usize::MAX {
			rand::random::<usize>() % SHAPES.len()
		} else {
			(*id + 1 + rand::random::<usize>() % (SHAPES.len() - 1)) % SHAPES.len()
		};
		*id
	});
	SHAPES[id]()
}

fn logistic(x: f64, a: f64, k: f64) -> f64 {
	1. / (1. + a * (-k * x).exp())
}

fn in_out_coefficient(x: f64) -> f64 {
	logistic(x, 500., 0.5)
}

impl Component for AnimationCanvas {
	type Message = Msg;
	type Properties = ();

	fn create(ctx: &Context<Self>) -> Self {
		let ctx = ctx.link().clone();

		Self {
			canvas: NodeRef::default(),
			time: 0,
			state: State::PopIn { timer: 0 },
			angle: array::from_fn(|_| rand::random::<f64>() * TAU),
			angle_velocity: [0.01, 0.02, 0.03],
			vertices: get_random_vertices(),
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
				web_sys::window()
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

		match &mut self.state {
			State::PopIn { timer } => {
				*timer += 1;
				if in_out_coefficient(*timer as f64 - 3. * self.vertices.len() as f64) > 1. - 1e-8 {
					self.state = State::Stable { timer: 0 };
				}
			}
			State::Stable { timer } => {
				*timer += 1;
				if *timer == State::STABLE_DURATION {
					self.state = State::PopOut { timer: 0 };
				}
			}
			State::PopOut { timer } => {
				*timer += 1;
				if in_out_coefficient(*timer as f64 - 3. * self.vertices.len() as f64) > 1. - 1e-8 {
					self.state = State::PopIn { timer: 0 };
					self.vertices = get_random_vertices();
				}
			}
		}

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
				let a = (i % 5) as f64;
				let b = (i % 3) as f64;
				((a * t).cos(), (b * t).cos())
			};

			let gray_scale = (208. + (x / 3. - y / 3. + 1.5 * z) * 32.).clamp(0f64, 255.) as u8;
			let co = z * 1.5 + 60.;
			let x = center.x + (x + 0.05 * dx) * co;
			let y = center.y + (y + 0.05 * dy) * co;
			let base_r = 8.3 + 1.25 * t.cos() + z / 2.5;
			let r = match self.state {
				State::PopIn { timer } => base_r * in_out_coefficient(timer as f64 - 3. * i as f64),
				State::PopOut { timer } => {
					base_r * (1. - in_out_coefficient(timer as f64 - 3. * i as f64))
				}
				_ => base_r,
			};
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
