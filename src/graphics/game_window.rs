pub use glfw::Key;
use glfw::{Action, Context, GlfwReceiver};
use nalgebra::Vector2;

use super::{
    render_device::RenderDevice,
    viewport::Viewport,
};

pub struct Window {
    pub glfw: glfw::Glfw,
    pub handle : glfw::PWindow,
    pub events : GlfwReceiver<(f64, glfw::WindowEvent)>,
    pub render_device: RenderDevice
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str, vsync: bool) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        let (mut handle, events) = glfw
            .create_window(
                width,
                height,
                title,
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");
        handle.set_key_polling(true);
        let mut render_device = RenderDevice::default();
        render_device.init(&mut handle);
        glfw.set_swap_interval(glfw::SwapInterval::Sync(vsync as u32));
        
        Self { glfw, handle, events, render_device}
    }

    pub fn make_current(&mut self) {
        self.handle.make_current();
    } 

    pub fn poll_events(&mut self) {
        self.glfw.poll_events();
    }

    pub fn swap_buffers(&mut self) {
        self.handle.swap_buffers();
    }

    pub fn flush_events(&mut self) -> Vec<(f64, glfw::WindowEvent)> {
        glfw::flush_messages(&self.events).collect()
    }

    pub fn should_close(&self) -> bool {
        self.handle.should_close()
    }

    pub fn get_viewport(&self) -> Viewport {
        Viewport {
            size : Vector2::new(self.handle.get_size().0 as u32, self.handle.get_size().1 as u32) 
        }
    }

    pub fn key_down(&self, key : Key) -> bool {
        let key = self.handle.get_key(key);
        match key {
           Action::Press => {
            return true;
           } 
           Action::Release => {
            return false;
           }
           Action::Repeat => {
            return false;
           }
        }
    }

}