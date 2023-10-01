use std::num::NonZeroU32;

use super::shape::Shape;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShapeId(NonZeroU32);

#[derive(Debug)]
pub struct Plan {
    next_id: NonZeroU32,
    shapes: Vec<(ShapeId, Shape)>,
}

impl Default for Plan {
    fn default() -> Self {
        Self {
            next_id: NonZeroU32::new(1).unwrap(),
            shapes: Default::default(),
        }
    }
}

impl Plan {
    pub fn generate_shape_id(&mut self) -> ShapeId {
        let id = self.next_id;
        self.next_id.checked_add(1).unwrap();
        ShapeId(id)
    }

    pub fn add_shape(&mut self, shape_id: ShapeId, shape: Shape) {
        self.shapes.push((shape_id, shape));
    }

    pub fn remove_shape(&mut self, shape_id: ShapeId) {
        self.shapes.retain(|(id, _)| *id != shape_id);
    }

    pub fn iter(&self) -> impl Iterator<Item = &(ShapeId, Shape)> {
        self.shapes.iter()
    }

    /// Check if the `shape` is disjoint from the rest of the shapes
    /// exepts the one with the id `ignore`
    pub fn is_disjoint(&self, shape: &Shape, ignore: Option<ShapeId>) -> bool {
        self.shapes
            .iter()
            .filter_map(|(id, shape)| match ignore {
                Some(ignore_id) if *id == ignore_id => None,
                _ => Some(shape),
            })
            .all(|s| s.is_disjoint(shape))
    }
}
