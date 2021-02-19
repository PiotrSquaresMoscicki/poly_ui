// std
use std::fmt::Debug;
// deps
use nalgebra::Point2;
use nalgebra::Vector2;
use uuid::Uuid;
// crate
use crate::poly_ui::app::PainterTrait;
use crate::poly_ui::components::Hierarchy;
use crate::poly_ui::components::Transform;
use crate::poly_ui::widgets::NewWidget;
use crate::poly_ui::widgets::Ownerless;
use crate::poly_ui::widgets::WidgetTrait;
// super
use super::Item;
use super::Layout;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct GridLayout {
    id: Uuid,
    hierarchy: Hierarchy,

    row_layout: Layout,
    is_row_layout_dirty: bool,
    column_layout: Layout,
    is_column_layout_dirty: bool,

    children_rows_columns: Vec<Vec<Option<Uuid>>>,
}

//************************************************************************************************
impl GridLayout {
    pub fn new_raw() -> Self {
        Self {
            id: Uuid::new_v4(),
            hierarchy: Hierarchy::default(),
            row_layout: Layout::new(0, vec!()),
            is_row_layout_dirty: true,
            column_layout: Layout::new(0, vec!()),
            is_column_layout_dirty: true,
            children_rows_columns: vec!(),
        }
    }

    pub fn new() -> NewWidget<Self> {
        NewWidget::new(Self::new_raw())
    }

    pub fn insert_child_at(&mut self, child: Ownerless, row: &Option<usize>, col: &Option<usize>) {
        let fixed_row = row.unwrap_or(self.row_layout.items.len());
        let fixed_col = col.unwrap_or(self.column_layout.items.len());
        self.ensure_row_count(fixed_row + 1);
        self.ensure_column_count(fixed_col+ 1);
        self.children_rows_columns[fixed_row][fixed_col] = Some(*child.borrow().id());
        self.hierarchy.add_with_transform(child, &Transform::default());
    }


    pub fn set_row_count(&mut self, size: usize) {
        self.row_layout.items.resize(size, Item::new(1, &None, &None));
        self.children_rows_columns.resize(size, vec!());
        self.is_row_layout_dirty = true;
    }

    fn ensure_row_count(&mut self, size: usize) {
        if self.row_layout.items.len() <= size {
            self.set_row_count(size);
        }
    }
    
    pub fn set_row_stretch(&mut self, row: u32, stretch: u32) {

    }
    
    pub fn set_row_max_size(&mut self, row: u32, size: u32) {

    }
    
    pub fn set_row_min_size(&mut self, row: u32, size: u32) {
    }
    

    pub fn set_column_count(&mut self, size: usize) {
        self.column_layout.items.resize(size, Item::new(1, &None, &None));
        for row in &mut self.children_rows_columns {
            row.resize(size, None);
        }
        self.is_column_layout_dirty = true;
    }

    fn ensure_column_count(&mut self, size: usize) {
        if self.column_layout.items.len() <= size {
            self.set_column_count(size);
        }
    }

    pub fn set_column_stretch(&mut self, col: u32, stretch: u32) {
        
    }
    
    pub fn set_column_max_size(&mut self, col: u32, size: u32) {
        
    }
    
    pub fn set_column_min_size(&mut self, col: u32, size: u32) {

    }

    fn refresh_children_transforms(&mut self, size: &Vector2<u32>) {
        self.row_layout.size = size.y;
        self.row_layout.refresh();
        self.column_layout.size = size.x;
        self.column_layout.refresh();

        let mut row_offset = 0;

        for row in 0..self.row_layout.items.len() {
            let row_size = self.row_layout.items[row].get_current_size();
            let mut col_offset = 0;

            for col in 0..self.column_layout.items.len() {
                let col_size = self.column_layout.items[col].get_current_size();

                match &self.children_rows_columns[row][col] {
                    Some(id) => self.hierarchy.set_transform(id, &Transform::new(
                        &Point2::<i32>::new(col_offset, row_offset), &Vector2::<u32>::new(col_size, row_size))),
                    None => ()
                }
                col_offset = col_offset + col_size as i32;
            }
            row_offset = row_offset + row_size as i32;
        }
    }
}

//************************************************************************************************
impl WidgetTrait for GridLayout {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn add_child(&mut self, child: Ownerless) {
        self.hierarchy.add(child);
    }

    fn remove_child(&mut self, child: &Uuid) -> Ownerless {
        self.hierarchy.remove(child)
    }

    fn get_hierarchy(&self) -> &Hierarchy {
        &self.hierarchy
    }

    fn get_child_transform(&self, child: &Uuid) -> &Transform {
        self.hierarchy.get_transform(child)
    }

    fn update(&mut self, dt: f32) {
        self.hierarchy.update_children(dt);
    }

    fn paint(&mut self, painter: &mut dyn PainterTrait) {
        let painter_size = painter.size();
        if self.is_row_layout_dirty 
            || self.is_column_layout_dirty 
            || self.row_layout.size != painter_size.y 
            || self.column_layout.size != painter_size.x {
            self.refresh_children_transforms(&painter_size)
        }

        self.hierarchy.paint_children(painter);
    }
}

#[cfg(test)]
mod tests {
    // crate
    use crate::poly_ui::widgets::MockWidget;
    use crate::poly_ui::app::MockPainter;
    // super
    use super::*;

    //********************************************************************************************
    #[test]
    fn insert_child_at() {
        let layout = GridLayout::new();
        let child = MockWidget::new();
        let child_ptr = child.get().clone();

        layout.borrow_mut().insert_child_at(child.make_ownerless(), &None, &None);

        assert_eq!(layout.borrow_mut().get_hierarchy().children().len(), 1);
        assert_eq!(
            layout.borrow_mut().get_hierarchy().children()[0].widget.borrow().id(), 
            child_ptr.borrow().id()
        );

        let mut painter = MockPainter::new();
        painter.size = Vector2::<u32>::new(50, 100);
        layout.borrow_mut().paint(&mut painter);
        Transform::new(&Point2::<i32>::new(0, 0), &Vector2::<u32>::new(0, 0));
        assert_eq!(
            layout.borrow_mut().get_child_transform(&child_ptr.borrow().id().clone()),
            &Transform::new(&Point2::<i32>::new(0, 0), &Vector2::<u32>::new(50, 100))
        );
    }
}
