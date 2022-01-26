use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use yew::{
    html,
    html::onclick,
    virtual_dom::{AttrValue, VTag},
    Callback, Html, MouseEvent,
};

use crate::{
    utils::{ManageChildren, VTagExt},
    Checkbox, MdcWidget, AUTO_INIT_ATTR,
};

pub mod mdc {
    pub const TYPE_NAME: &str = "MDCDataTable";
}

pub enum TableCell {
    Numeric(Html),
    Text(Html),
}

impl TableCell {
    pub fn num(content: impl Into<Html>) -> Self {
        TableCell::Numeric(content.into())
    }

    pub fn text(content: impl Into<Html>) -> Self {
        TableCell::Text(content.into())
    }

    pub fn content(&self) -> &Html {
        match self {
            TableCell::Numeric(content) => content,
            TableCell::Text(content) => content,
        }
    }

    fn build_head_cell(self) -> Html {
        let (class, content) = match self {
            TableCell::Numeric(content) => (
                "mdc-data-table__header-cell mdc-data-table__header-cell--numeric",
                content,
            ),
            TableCell::Text(content) => ("mdc-data-table__header-cell", content),
        };
        html! {
            <th class = { class } role = "columnheader" scope = "col">{ content }</th>
        }
    }

    fn build_body_cell(self) -> Html {
        let (class, content) = match self {
            TableCell::Numeric(content) => ("mdc-data-table__cell mdc-data-table__cell--numeric", content),
            TableCell::Text(content) => ("mdc-data-table__cell", content),
        };
        html! {
            <td class = { class }>{ content }</td>
        }
    }
}

pub type OnRowClickFn = fn(&VTag) -> Callback<MouseEvent>;

#[derive(Clone)]
pub struct DataTable {
    html: Html,
    row_selection: bool,
    on_row_click: Option<OnRowClickFn>,
}

impl DataTable {
    pub fn new(id: impl Into<String>) -> Self {
        let id = id.into();
        let mut table = Self {
            html: html! {
                <div id = { id } class = "mdc-data-table">
                    <div class = "mdc-data-table__table-container">
                        <table class = "mdc-data-table__table">
                            <thead>
                                <tr class = "mdc-data-table__header-row"></tr>
                            </thead>
                            <tbody class = "mdc-data-table__content">
                            </tbody>
                        </table>
                    </div>
                </div>
            },
            row_selection: false,
            on_row_click: None,
        };
        table.root_tag_mut().set_attr(AUTO_INIT_ATTR, mdc::TYPE_NAME);
        table
    }

    pub fn head(mut self, head: impl IntoIterator<Item = TableCell>) -> Self {
        let head_cells: Vec<Html> = head.into_iter().map(|cell| cell.build_head_cell()).collect();

        if let Some(header_cols) = self.table_header_row_tag_mut().children_mut() {
            for idx in 0..header_cols.len() {
                if !header_cols[idx].is_contains_class("mdc-data-table__header-cell--checkbox") {
                    header_cols.remove(idx);
                }
            }

            for cell in head_cells {
                header_cols.push(cell);
            }
        }
        self
    }

    pub fn row(mut self, row: impl IntoIterator<Item = TableCell>) -> Self {
        let row: Vec<_> = row.into_iter().map(|cell| cell.build_body_cell()).collect();

        let row_id = format!("{}-row-{}", self.root_id(), self.row_count());
        let mut row = html! {
            <tr data-row-id = { row_id.clone() } class = "mdc-data-table__row">{ row }</tr>
        };

        let row_checkbox = if self.row_selection {
            Some(Self::row_checkbox(row_id.clone()))
        } else {
            None
        };

        if let Some(cell) = row.find_child_tag_mut("td") {
            cell.set_attr("scope", "row");
            cell.set_attr("id", row_id);
        }

        if let Some(row_checkbox) = row_checkbox {
            if let Html::VTag(row) = &mut row {
                if let Some(cells) = row.children_mut() {
                    cells.insert(0, row_checkbox);
                }
            }
        }

        if let Some(on_row_click) = self.on_row_click {
            Self::add_on_click_to_row(&mut row, on_row_click);
        }

        if let Some(rows) = self.table_body_tag_mut().children_mut() {
            rows.push(row);
        }
        self
    }

    pub fn on_row_click(mut self, on_row_click: OnRowClickFn) -> Self {
        self.on_row_click = Some(on_row_click);

        let body = self.table_body_tag_mut();
        if let Some(children) = body.children_mut() {
            for row in children.iter_mut() {
                Self::add_on_click_to_row(row, on_row_click);
            }
        }
        self
    }

    pub fn row_selection(mut self, selection: bool) -> Self {
        if selection && !self.row_selection {
            self.row_selection = true;

            let mut head_checkbox = Self::head_checkbox(self.root_id());
            if let Some(input) = head_checkbox.find_child_tag_mut("input") {
                input.set_attr("aria-label", "Toggle all rows");
            }

            let head_row = self.table_header_row_tag_mut();
            head_row.insert_child(0, head_checkbox);

            let body = self.table_body_tag_mut();
            if let Some(children) = body.children_mut() {
                for row in children.iter_mut() {
                    if let Html::VTag(row) = row {
                        let row_id = row.attr("id").expect("A row ID expected").clone();
                        row.insert_child(0, Self::row_checkbox(row_id));
                    }
                }
            }
        } else if !selection && self.row_selection {
            self.row_selection = selection;

            let head_row = self.table_header_row_tag_mut();
            if head_row.is_first_child_contains_class("mdc-data-table__header-cell--checkbox") {
                head_row.remove_child(0);
            }

            let body = self.table_body_tag_mut();
            if let Some(children) = body.children_mut() {
                for row in children.iter_mut() {
                    if row.is_first_child_contains_class("mdc-data-table__cell--checkbox") {
                        if let Html::VTag(ref mut row) = row {
                            row.remove_child(0);
                        }
                    }
                }
            }
        }
        self
    }

    pub fn row_count(&self) -> usize {
        self.table_body_tag().children().len()
    }

    pub fn root_id(&self) -> AttrValue {
        if let Html::VTag(tag) = &self.html {
            tag.attr("id").expect("The DataTable widget must have ID")
        } else {
            panic!("The DataTable widget must be contains the root tag!")
        }
    }

    pub fn table_tag(&self) -> &VTag {
        if let Html::VTag(tag) = &self.html {
            if let Some(Html::VTag(tag)) = tag.children().first() {
                if let Some(table) = tag.find_child_tag("table") {
                    return table;
                }
            }
        }
        panic!("The DataTable widget must be contains the table tag!");
    }

    fn table_tag_mut(&mut self) -> &mut VTag {
        if let Html::VTag(tag) = &mut self.html {
            if let Some(tag) = tag.first_child_tag_mut() {
                if let Some(table) = tag.find_child_tag_mut("table") {
                    return table;
                }
            }
        }
        panic!("The DataTable widget must be contains the table tag!");
    }

    fn table_header_row_tag_mut(&mut self) -> &mut VTag {
        if let Some(head) = self.table_tag_mut().find_child_tag_mut("thead") {
            if let Some(row) = head.find_child_tag_mut("tr") {
                return row;
            }
        }
        panic!("The DataTable widget must be contains the table header row tag!");
    }

    pub fn table_body_tag(&self) -> &VTag {
        match self.table_tag().children().get(1) {
            Some(Html::VTag(tag)) if tag.tag() == "tbody" => tag,
            _ => panic!("The DataTable widget must be contains the table body tag!"),
        }
    }

    fn table_body_tag_mut(&mut self) -> &mut VTag {
        match self
            .table_tag_mut()
            .children_mut()
            .and_then(|children| children.get_mut(1))
        {
            Some(Html::VTag(tag)) if tag.tag() == "tbody" => tag,
            _ => panic!("The DataTable widget must be contains the table body tag!"),
        }
    }

    fn add_on_click_to_row(row: &mut Html, on_row_click: OnRowClickFn) -> bool {
        if let Html::VTag(tag) = row {
            let callback = on_row_click(tag);
            tag.add_listener(Rc::new(onclick::Wrapper::new(callback)))
        } else {
            false
        }
    }

    fn head_checkbox(root_id: impl AsRef<str>) -> Html {
        let root_id = root_id.as_ref();
        let checkbox = Checkbox::new()
            .id(format!("{}-header-checkbox", root_id))
            .class("mdc-data-table__header-row-checkbox mdc-checkbox--selected");
        html! {
            <th class = "mdc-data-table__header-cell mdc-data-table__header-cell--checkbox" role = "columnheader" scope = "col">
                { checkbox }
            </th>
        }
    }

    fn row_checkbox(row_id: impl Into<AttrValue>) -> Html {
        let row_id = row_id.into();
        let checkbox = Checkbox::new()
            .id(format!("{}-checkbox", row_id))
            .class("mdc-data-table__row-checkbox")
            .labeled_by(row_id);
        html! {
            <td class = "mdc-data-table__cell mdc-data-table__cell--checkbox">
                { checkbox }
            </td>
        }
    }
}

impl MdcWidget for DataTable {
    const NAME: &'static str = stringify!(DataTable);

    fn html(&self) -> &Html {
        &self.html
    }

    fn html_mut(&mut self) -> &mut Html {
        &mut self.html
    }
}

impl Deref for DataTable {
    type Target = Html;

    fn deref(&self) -> &Self::Target {
        &self.html
    }
}

impl DerefMut for DataTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.html
    }
}

impl From<DataTable> for Html {
    fn from(widget: DataTable) -> Self {
        widget.html
    }
}
