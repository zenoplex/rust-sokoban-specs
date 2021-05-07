use crate::components::*;
use crate::constants::*;
use ggez::{graphics, nalgebra, Context};
use specs::{Join, ReadStorage, System};

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables) = data;

        graphics::clear(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        let mut rendering_data: Vec<_> = (&positions, &renderables).join().collect();
        // sort by z-index
        rendering_data.sort_by_key(|&k| k.0.z);

        for (position, renderable) in rendering_data.iter() {
            let image =
                graphics::Image::new(self.context, renderable.path.clone()).expect("Image error");
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;

            let draw_params = graphics::DrawParam::new().dest(nalgebra::Point2::new(x, y));
            graphics::draw(self.context, &image, draw_params).expect("Draw error");
        }

        graphics::present(self.context).expect("Render error");
    }
}
