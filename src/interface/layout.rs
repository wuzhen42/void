use super::panel::{Panel, Widget};
use crate::prim::*;
use winit::event::VirtualKeyCode;

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
    const MARGIN: f64 = 0.02;

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
        let min_start = rect.bottomleft();
        let max_start = match self.orient {
            Orientation::Horizontal => rect.topleft(),
            Orientation::Vertical => rect.bottomright(),
        };
        for (span, child) in left.windows(2).zip(self.children.iter_mut()) {
            let is_first = span[0] == 0.0;
            let is_last = span[1] == 1.0;
            let child_rect = Rect::from_corner(
                min_start + offset * (span[0] + if is_first { 0.0 } else { 0.5 * Layout::MARGIN }),
                max_start + offset * (span[1] - if is_last { 0.0 } else { 0.5 * Layout::MARGIN }),
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

    pub fn filter_map<B, F>(&mut self, f: F) -> Vec<B>
    where
        F: Fn(&mut Box<dyn Panel>) -> Option<B> + Copy,
    {
        let mut results = vec![];
        self.children.iter_mut().for_each(|child| match child {
            Node::Inner(layout) => {
                results.extend(layout.filter_map(f));
            }
            Node::Leaf(panel) => {
                if let Some(x) = f(panel) {
                    results.push(x);
                }
            }
        });
        results
    }
}

impl Widget for Layout {
    fn resize(&mut self, rect: Rect) {
        self.for_each(|x| x.resize(rect));
    }

    fn onclick(&mut self, cursor: Pnt2) -> bool {
        if !self.rect.contains(cursor) {
            false
        } else {
            self.children.iter_mut().any(|child| match child {
                Node::Inner(layout) => layout.onclick(cursor),
                Node::Leaf(panel) => panel.onclick(cursor),
            })
        }
    }

    fn onkeydown(&mut self, key: VirtualKeyCode) -> bool {
        self.children.iter_mut().any(|child| match child {
            Node::Inner(layout) => layout.onkeydown(key),
            Node::Leaf(panel) => panel.onkeydown(key),
        })
    }
}
