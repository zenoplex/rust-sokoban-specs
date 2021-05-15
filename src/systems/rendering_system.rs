use crate::{components::*, resources::Gameplay};
use crate::{constants::*, resources::Time};
use ggez::{
    graphics::{self, spritebatch::SpriteBatch, Color, DrawParam, Image},
    nalgebra, timer, Context,
};
use itertools::Itertools;
use specs::{Join, Read, ReadStorage, System};
use std::{collections::HashMap, time::Duration};

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (
        Read<'a, Gameplay>,
        Read<'a, Time>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (gameplay, time, positions, renderables) = data;

        graphics::clear(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        let rendering_data: Vec<_> = (&positions, &renderables).join().collect();
        // {z: u8, map: { path: string, draw_params: [] }}
        let mut rendering_batches: HashMap<u8, HashMap<String, Vec<DrawParam>>> = HashMap::new();

        for (position, renderable) in rendering_data.iter() {
            let image = self.get_image(&renderable, time.delta);
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;
            let z = position.z;

            let draw_params = graphics::DrawParam::new().dest(nalgebra::Point2::new(x, y));
            rendering_batches
                .entry(z)
                .or_default()
                .entry(image)
                .or_default()
                .push(draw_params);
        }

        for (_z, group) in rendering_batches
            .iter()
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
        {
            for (image_path, draw_params) in group {
                let image = Image::new(self.context, image_path).expect("Image error");
                let mut sprite_batch = SpriteBatch::new(image);

                for draw_param in draw_params.iter() {
                    sprite_batch.add(*draw_param);
                }

                graphics::draw(self.context, &sprite_batch, graphics::DrawParam::new())
                    .expect("Draw error");
            }
        }

        self.draw_text(&gameplay.state.to_string(), 525.0, 80.0);
        self.draw_text(&gameplay.moves_count.to_string(), 525.0, 100.0);

        // fps meter
        let fps = format!("fps: {:.0}", timer::fps(self.context));
        self.draw_text(&fps, 525.0, 120.0);

        graphics::present(self.context).expect("Render error");
    }
}

impl RenderingSystem<'_> {
    pub fn draw_text(&mut self, text_string: &str, x: f32, y: f32) {
        let text = graphics::Text::new(text_string);
        let destination = nalgebra::Point2::new(x, y);
        let color = Some(Color::new(0.0, 0.0, 0.0, 1.0));
        let dimentions = nalgebra::Point2::new(0.0, 20.0);

        graphics::queue_text(self.context, &text, dimentions, color);
        graphics::draw_queued_text(
            self.context,
            graphics::DrawParam::new().dest(destination),
            None,
            graphics::FilterMode::Linear,
        )
        .expect("Draw error");
    }

    pub fn get_image(&mut self, renderable: &Renderable, delta: Duration) -> String {
        let path_index = match renderable.kind() {
            RenderableKind::Static => 0,
            RenderableKind::Animated => ((delta.as_millis() % 1000) / 250) as usize,
        };

        renderable.path(path_index)
    }
}
