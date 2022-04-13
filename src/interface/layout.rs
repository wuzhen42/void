use super::panel::Panel;
use crate::prim::{Rect, Vec2};

pub enum Orientation {
    Horizontal,
    Vertical,
}

enum Node {
    Inner(Layout),
    Leaf(Box<dyn Panel>),
}

pub struct Layout {
    children: Vec<Node>,
    weights: Vec<f64>,
    orient: Orientation,
    rect: Rect,
}

impl Layout {
    pub fn new(rect: Rect, orient: Orientation) -> Layout {
        Layout {
            children: vec![],
            weights: vec![],
            orient,
            rect,
        }
    }

    pub fn grow(&mut self, panel: Box<dyn Panel>) {
        self.grow_with_weight(panel, 1.0);
    }

    pub fn grow_with_weight(&mut self, panel: Box<dyn Panel>, w: f64) {
        self.children.push(Node::Leaf(panel));
        self.weights.push(w);
        self.resize(self.rect);
    }

    pub fn push(&mut self, child: Layout) {
        self.push_with_weight(child, 1.0);
    }

    pub fn push_with_weight(&mut self, child: Layout, w: f64) {
        self.children.push(Node::Inner(child));
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
        let max_start = match self.orient {
            Orientation::Horizontal => rect.topleft(),
            Orientation::Vertical => rect.bottomright(),
        };
        for (span, child) in left.windows(2).zip(self.children.iter_mut()) {
            let child_rect = Rect::from_corner(
                rect.bottomleft() + offset * span[0],
                max_start + offset * span[1],
            );
            match child {
                Node::Leaf(leaf) => leaf.resize(child_rect),
                Node::Inner(layout) => layout.resize(rect),
            }
        }
    }

    pub fn for_each<F>(&mut self, f: F)
    where
        F: Fn(&mut Box<dyn Panel>) + Copy,
    {
        self.children.iter_mut().for_each(|child| match child {
            Node::Inner(layout) => {
                layout.for_each(f);
            }
            Node::Leaf(panel) => {
                f(panel);
            }
        });
    }

    pub fn map<B, F>(&mut self, f: F) -> Vec<B>
    where
        F: Fn(&mut Box<dyn Panel>) -> B + Copy,
    {
        let mut results = vec![];
        self.children.iter_mut().for_each(|child| match child {
            Node::Inner(layout) => {
                results.extend(layout.map(f));
            }
            Node::Leaf(panel) => {
                results.push(f(panel));
            }
        });
        results
    }
}
