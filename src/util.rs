use rapier2d::{math::Real, na::Matrix2x1};
use sfml::{
    graphics::{RenderStates, RenderTarget},
    system::Vector2,
};

pub trait Drawable {
    fn draw(&mut self, target: &mut dyn RenderTarget, states: &RenderStates);
}

pub trait ToNaMat2x1 {
    fn to_na_mat2x1(&self) -> Matrix2x1<Real>;
}

impl ToNaMat2x1 for Vector2<f32> {
    fn to_na_mat2x1(&self) -> Matrix2x1<Real> {
        Matrix2x1::new(self.x, self.y)
    }
}

macro_rules! assets {
    ($path:literal) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/assets/", $path)
    };
}

pub(crate) use assets;

#[test]
fn test_assets() {
    println!("{}", assets!("bounce.wav"));
}
