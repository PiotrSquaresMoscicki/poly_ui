use nalgebra::Vector2;
use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};
use uuid::Uuid;

use super::Layout;
use crate::poly_ui::components::Hierarchy;
use crate::poly_ui::widgets::WidgetTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct CanvasLayout {
    children: HashMap<Uuid, Vector2<i32>>,
    hierarchy: Rc<RefCell<Hierarchy>>,
}

//************************************************************************************************
impl CanvasLayout {
    pub fn new() -> Self {
        return Self {
            children: HashMap::new(),
            hierarchy: Rc::new(RefCell::new(Hierarchy::new())),
        };
    }
}

//************************************************************************************************
impl Layout for CanvasLayout {
    fn set_owner_widget_hierarchy(&mut self, hierarchy: Rc<RefCell<Hierarchy>>) {
        self.hierarchy = hierarchy;
    }

    fn add(&mut self, child: Rc<RefCell<dyn WidgetTrait>>, pos: Vector2<i32>) {
        self.children.insert(*child.borrow().id(), pos);
        self.hierarchy.borrow_mut().add(child);
    }
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[cfg(test)]
mod tests {
    use nalgebra::Vector2;
    use std::{cell::RefCell, rc::Rc};

    use crate::poly_ui::layouts::CanvasLayout;
    use crate::poly_ui::widgets::{Widget, WidgetTrait};

    //********************************************************************************************
    #[test]
    fn canvas_layout_add_child() {
        let mut parent_widget = Widget::new();
        parent_widget.set_layout(Box::new(CanvasLayout::new()));
        let child_widget = Rc::new(RefCell::new(Widget::new()));
        parent_widget
            .layout_mut()
            .add(child_widget.clone(), Vector2::<i32>::new(1, 2));

        assert_eq!(
            parent_widget.hierarchy().children()[0].borrow().id(),
            child_widget.borrow().id()
        );
    }
}
