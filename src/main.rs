extern crate winit;

fn main() {
	// create events loop, resizable flag, and window
	let mut events_loop = winit::EventsLoop::new();
	let mut resizable = false;
	let window = winit::WindowBuilder::new()
		.with_title("Hit space to toggle resizability.")
		.with_dimensions((400,200).into())
		.build(&events_loop)
		.unwrap();
		
	// ?? research ??
	events_loop.run_forever(|event| {
		match event {
			winit::Event::WindowEvent { event, ..} => match event {
				winit::WindowEvent::CloseRequested => return winit::ControlFlow::Break,
				winit::WindowEvent::KeyboardInput {
					input:
						winit::KeyboardInput {
							virtual_keycode: Some(winit::VirtualKeyCode::Space),
							state: winit::ElementState::Released,
							..
						},
						..
				} => {
					resizable = !resizable;
					println!("Resizable: {}", resizable);
					window.set_resizable(resizable);
				}
				_ => (),
			},
			_ => (),
		};
		winit::ControlFlow::Continue
	});
}
