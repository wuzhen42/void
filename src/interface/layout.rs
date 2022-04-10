use crate::prim::{Rect, Vec2};

#[derive(Debug, PartialEq)]
enum Orientation {
    Horizontal,
    Vertical,
}

enum Node {
    Inner(Layout),
    Leaf(Rect),
}

pub struct Layout {
    children: Vec<Node>,
    weights: Vec<f64>,
    orient: Orientation,
    rect: Rect,
}

impl Layout {
    pub fn new(rect: Rect) -> Layout {
        Layout {
            children: vec![],
            weights: vec![],
            orient: Orientation::Horizontal,
            rect,
        }
    }

    pub fn grow(&mut self) {
        self.grow_with_weight(1.0);
    }

    pub fn grow_with_weight(&mut self, w: f64) {
        self.children.push(Node::Leaf(Rect::empty()));
        self.weights.push(w);
        self.resize(self.rect);
    }

    fn resize(&mut self, rect: Rect) {
        let total_weight = self.weights.iter().sum::<f64>();
        let width = self.weights.iter().map(|x| x / total_weight);
        let left: Vec<f64> = std::iter::once(0.0)
            .chain(width.scan(0.0, |acc, x| {
                *acc += x;
                Some(*acc)
            }))
            .collect();

        let offset = match self.orient {
            Orientation::Horizontal => Vec2::new(1.0, 0.0),
            Orientation::Vertical => Vec2::new(0.0, 1.0),
        } * rect.extent();
        for (span, child) in left.windows(2).zip(self.children.iter_mut()) {
            let child_rect = Rect::from_corner(
                rect.bottomleft() + offset * span[0],
                rect.topleft() + offset * span[1],
            );
            match child {
                Node::Leaf(leaf) => *leaf = child_rect,
                Node::Inner(layout) => layout.resize(rect),
            }
        }
    }

    pub fn leaves(&self) -> Vec<Rect> {
        let mut result = vec![];
        for child in self.children.iter() {
            match child {
                Node::Inner(layout) => {
                    result.extend(layout.leaves());
                }
                Node::Leaf(rect) => {
                    result.push(*rect);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prim::Pnt2;

    #[test]
    fn test_resize_average() {
        let mut root = Layout::new(Rect::from_corner(
            Pnt2::new(-1.0, -1.0),
            Pnt2::new(1.0, 1.0),
        ));
        root.grow();
        root.grow();
        let leaves = root.leaves();
        assert_eq!(leaves.len(), 2);
        assert_eq!(leaves[0].min, Pnt2::new(-1.0, -1.0));
        assert_eq!(leaves[0].max, Pnt2::new(0.0, 1.0));
        assert_eq!(leaves[1].min, Pnt2::new(0.0, -1.0));
        assert_eq!(leaves[1].max, Pnt2::new(1.0, 1.0));
    }

    #[test]
    fn test_resize_weighted() {
        let mut root = Layout::new(Rect::from_corner(
            Pnt2::new(-1.0, -1.0),
            Pnt2::new(1.0, 1.0),
        ));
        root.grow_with_weight(1.0);
        root.grow_with_weight(2.0);
        root.grow_with_weight(1.0);

        let leaves = root.leaves();
        assert_eq!(leaves.len(), 3);
        assert_eq!(leaves[0].min, Pnt2::new(-1.0, -1.0));
        assert_eq!(leaves[0].max, Pnt2::new(-0.5, 1.0));
        assert_eq!(leaves[1].min, Pnt2::new(-0.5, -1.0));
        assert_eq!(leaves[1].max, Pnt2::new(0.5, 1.0));
        assert_eq!(leaves[2].min, Pnt2::new(0.5, -1.0));
        assert_eq!(leaves[2].max, Pnt2::new(1.0, 1.0));
    }
}
