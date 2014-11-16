extern crate glutin;
extern crate libc;
extern crate gl;



fn main() {
    draw_window()
}

fn draw_window() {
  /// window drawing util is glutin from...  https://github.com/tomaka/glutin
    unsafe {
        let window = glutin::WindowBuilder::new()
            .with_title("Red Centipede 0.0.1-alpha".to_string())
            .build()
            .unwrap();

        window.make_current();

        gl::load_with(|symbol| window.get_proc_address(symbol));

        gl::ClearColor(0.0, 1.0, 0.0, 1.0); 

        while !window.is_closed() {
            window.wait_events();

            gl::Clear(gl::COLOR_BUFFER_BIT);

            window.swap_buffers();
        }
    }
}