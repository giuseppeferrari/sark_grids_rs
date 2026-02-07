//! A trait for types representing the size of a rectangular 2d grid.
use glam::{IVec2, USizeVec2, UVec2};

use crate::GridPoint;

/// A trait for types representing the size of a rectangular 2d grid.
pub trait GridSize: Clone + GridPoint {
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn size(&self) -> USizeVec2 {
        USizeVec2::new(self.width(), self.height())
    }

    /// The number of tiles for a grid of this size.
    fn tile_count(&self) -> usize {
        self.width() * self.height()
    }
    fn contains_point(&self, xy: impl super::GridPoint) -> bool {
        let xy = xy.to_ivec2();
        xy.cmpge(IVec2::ZERO).all() && xy.cmplt(self.to_ivec2()).all()
    }
}

macro_rules! impl_grid_size {
    ($type:ty) => {
        impl GridSize for $type {
            fn width(&self) -> usize {
                self[0] as usize
            }

            fn height(&self) -> usize {
                self[1] as usize
            }
        }
    };
}

macro_rules! impl_grid_tuple_size {
    ($type:ty) => {
        impl GridSize for ($type, $type) {
            fn width(&self) -> usize {
                self.0 as usize
            }

            fn height(&self) -> usize {
                self.1 as usize
            }
        }
    };
}

impl_grid_size!(UVec2);
impl_grid_size!(USizeVec2);
impl_grid_size!([u32; 2]);
impl_grid_size!([usize; 2]);
impl_grid_tuple_size!(u32);
impl_grid_tuple_size!(usize);
