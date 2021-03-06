use rustation::gpu::renderer::{Renderer, Vertex, PrimitiveAttributes};
use retrogl::{State, DrawConfig};

/// RetroGL state when no OpenGL context is available. It just holds
/// the data necessary to restart the emulation when a new context is
/// provided.
pub struct DummyState {
    config: DrawConfig,
}

impl DummyState {
    pub fn from_config(config: DrawConfig) -> DummyState {
        DummyState {
            config: config,
        }
    }
}

impl State for DummyState {
    fn draw_config(&self) -> &DrawConfig {
        &self.config
    }

    fn refresh_variables(&mut self) -> bool {
        false
    }

    fn renderer_mut(&mut self) -> &mut Renderer {
        &mut *self
    }

    fn prepare_render(&mut self) {
    }

    fn finalize_frame(&mut self) {
    }
}

impl Renderer for DummyState {
    fn set_draw_offset(&mut self, x: i16, y: i16) {
        self.config.draw_offset = (x, y)
    }

    fn set_draw_area(&mut self, top_left: (u16, u16), dimensions: (u16, u16)) {
        self.config.draw_area_top_left = top_left;
        self.config.draw_area_dimensions = dimensions;
    }

    fn set_display_mode(&mut self,
                        top_left: (u16, u16),
                        resolution: (u16, u16),
                        depth_24bpp: bool) {
        self.config.display_top_left = top_left;
        self.config.display_resolution = resolution;
        self.config.display_24bpp = depth_24bpp;
    }

    fn push_line(&mut self, _: &PrimitiveAttributes, _: &[Vertex; 2]) {
        warn!("Dummy push_line called");
    }

    fn push_triangle(&mut self, _: &PrimitiveAttributes, _: &[Vertex; 3]) {
        warn!("Dummy push_triangle called");
    }

    fn push_quad(&mut self, _: &PrimitiveAttributes, _: &[Vertex; 4]) {
        warn!("Dummy push_quad called");
    }

    fn fill_rect(&mut self, _: [u8; 3], _: (u16, u16), _: (u16, u16)) {
        warn!("Dummy fill_rect called");
    }

    fn load_image(&mut self, _: (u16, u16), _: (u16, u16), _: &[u16]) {
        warn!("Dummy load_image called");
    }
}
