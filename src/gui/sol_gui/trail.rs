use bevy::prelude::*;

pub struct Trail(VecDeque<Vector2D<f32>>);

impl Trail {
    pub fn new() -> Trail {
        Trail(VecDeque::new())
    }

    pub fn push(&mut self, pos: Vector2D<f32>) {
        self.0.push_back(pos);
    }

    pub fn pop(&mut self) {
        self.0.pop_front();
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    // pub fn draw(&self, ctx: &mut Context, color: Color) -> GameResult<()> {
    //     let mut mesh = MeshBuilder::new();
    //     for pos in &self.0 {
    //         mesh.circle(DrawMode::fill(), pos, 1.0, 0.1, color)?;
    //     }
    //     let mesh = mesh.build(ctx)?;
    //     graphics::draw(ctx, &mesh, DrawParam::default())
    // }
}