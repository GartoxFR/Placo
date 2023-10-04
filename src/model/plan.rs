use std::num::NonZeroU32;

use super::shape::Shape;
use super::vec2::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(test, derive(Hash))]
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
        self.next_id = self.next_id.checked_add(1).unwrap();
        ShapeId(id)
    }

    pub fn add_shape(&mut self, shape_id: ShapeId, shape: Shape) {
        self.shapes.push((shape_id, shape));
    }

    pub fn get_shape(&self, shape_id: ShapeId) -> Option<&Shape> {
        self.shapes
            .iter()
            .find_map(|(id, shape)| Some(shape).filter(|_| shape_id == *id))
    }

    pub fn get_shape_at(&self, point: &Vec2) -> Option<&(ShapeId, Shape)> {
        self.shapes.iter().find(|(_, shape)| shape.contains(point))
    }

    pub fn remove_shape(&mut self, shape_id: ShapeId) {
        self.shapes.retain(|(id, _)| *id != shape_id);
    }

    pub fn replace_shape(&mut self, shape_id: ShapeId, shape: Shape) {
        self.remove_shape(shape_id);
        self.shapes.push((shape_id, shape));
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

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn id_all_unique() {
        let mut plan = Plan::default();
        let ids: Vec<_> = std::iter::repeat_with(|| plan.generate_shape_id())
            .take(200)
            .collect();
        let total_id_count = ids.len();
        let id_set: HashSet<_> = ids.into_iter().collect();
        let unique_id_count = id_set.len();

        assert_eq!(total_id_count, unique_id_count);
    }
}
