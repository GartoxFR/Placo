use super::shape::Shape;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShapeId(u32);

#[derive(Debug, Default)]
pub struct Plan {
    next_id: u32,
    shapes: Vec<(ShapeId, Shape)>
}

impl Plan {
    pub fn generate_shape_id(&mut self) -> ShapeId {
        let id = self.next_id;
        self.next_id += 1;
        ShapeId(id)
    }

    pub fn add_shape(&mut self, shape_id: ShapeId, shape: Shape) {
        self.shapes.push((shape_id, shape));
    }

    pub fn remove_shape(&mut self, shape_id: ShapeId) {
        self.shapes.retain(|(id, _)| *id != shape_id );
    }

    pub fn iter(&self) -> impl Iterator<Item = &(ShapeId, Shape)> {
        self.shapes.iter()
    }
}


