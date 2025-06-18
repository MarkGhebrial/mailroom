use std::rc::Rc;

use ratatui::{
    style::{Style, Stylize},
    widgets::{Block, List, Widget},
};

pub trait Node {
    fn parent(&self) -> &Option<Rc<dyn Node>>;
    fn children(&self) -> &Vec<Rc<dyn Node>>;
    fn name(&self) -> &str;
}

pub struct SimpleNode {
    parent: Option<Rc<dyn Node>>,
    children: Vec<Rc<dyn Node>>,
    name: String,
}

impl SimpleNode {
    fn new(parent: Option<Rc<dyn Node>>, name: String) -> Self {
        Self {
            parent,
            children: Vec::new(),
            name,
        }
    }

    fn child(mut self, child: Rc<dyn Node>) -> Self {
        self.children.push(child);
        self
    }
}

impl Node for SimpleNode {
    fn parent(&self) -> &Option<Rc<dyn Node>> {
        &self.parent
    }

    fn children(&self) -> &Vec<Rc<dyn Node>> {
        &self.children
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Widget for &SimpleNode {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        // let items = ["Item 1", "Item 2", "Item 3"];
        let list = List::new(self.children().iter().map(|c| c.name()))
            .block(Block::bordered().title(self.name()))
            .highlight_style(Style::new().reversed())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);

        list.render(area, buf);
    }
}
