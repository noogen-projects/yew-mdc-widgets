use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use yew::{html, html::onclick, virtual_dom::VTag, Callback, Html, MouseEvent};

use crate::{utils::VTagExt, Checkbox, Text};

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
            <th class = class role = "columnheader" scope = "col">{ content }</th>
        }
    }

    fn build_body_cell(self) -> Html {
        let (class, content) = match self {
            TableCell::Numeric(content) => ("mdc-data-table__cell mdc-data-table__cell--numeric", content),
            TableCell::Text(content) => ("mdc-data-table__cell", content),
        };
        html! {
            <td class = class>{ content }</td>
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
    pub fn new<'a>(id: impl Into<Text<'a>>) -> Self {
        let id = id.into();
        let init_table = format!("mdc.dataTable.MDCDataTable.attachTo(document.getElementById('{}'))", id);

        Self {
            html: html! {
                <div id = id class = "mdc-data-table">
                    <div class = "mdc-data-table__table-container">
                        <table class = "mdc-data-table__table">
                            <thead>
                                <tr class = "mdc-data-table__header-row"></tr>
                            </thead>
                            <tbody class = "mdc-data-table__content">
                            </tbody>
                        </table>
                    </div>
                    <script>{ init_table }</script>
                </div>
            },
            row_selection: false,
            on_row_click: None,
        }
    }

    pub fn head(mut self, head: impl IntoIterator<Item = TableCell>) -> Self {
        let head_cells: Vec<Html> = head.into_iter().map(|cell| cell.build_head_cell()).collect();

        let head_row = self.table_header_row_tag_mut();
        for idx in 0..head_row.children.len() {
            if !head_row.children[idx].is_contains_class("mdc-data-table__header-cell--checkbox") {
                head_row.children.remove(idx);
            }
        }

        for cell in head_cells {
            head_row.children.push(cell);
        }
        self
    }

    pub fn row(mut self, row: impl IntoIterator<Item = TableCell>) -> Self {
        let row: Vec<_> = row.into_iter().map(|cell| cell.build_body_cell()).collect();

        let row_id = format!("{}-row-{}", self.root_id(), self.row_count());
        let mut row = html! {
            <tr data-row-id = row_id class = "mdc-data-table__row">{ row }</tr>
        };

        let row_checkbox = if self.row_selection {
            Some(Self::row_checkbox(&row_id))
        } else {
            None
        };

        if let Some(cell) = row.first_child_tag_mut("td") {
            cell.attributes.insert("scope".into(), "row".into());
            cell.attributes.insert("id".into(), row_id);
        }

        if let Some(row_checkbox) = row_checkbox {
            if let Html::VTag(row) = &mut row {
                row.children.insert(0, row_checkbox);
            }
        }

        if let Some(on_row_click) = self.on_row_click {
            Self::add_on_click_to_row(&mut row, on_row_click);
        }

        self.table_body_tag_mut().children.push(row);
        self
    }

    pub fn on_row_click(mut self, on_row_click: OnRowClickFn) -> Self {
        self.on_row_click = Some(on_row_click);

        let body = self.table_body_tag_mut();
        for row in body.children.iter_mut() {
            Self::add_on_click_to_row(row, on_row_click);
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
            head_row.children.insert(0, head_checkbox);

            let body = self.table_body_tag_mut();
            for row in body.children.iter_mut() {
                if let Html::VTag(row) = row {
                    let row_id = row.attributes.get("id").expect("A row ID expected");
                    row.children.insert(0, Self::row_checkbox(row_id));
                }
            }
        } else if !selection && self.row_selection {
            self.row_selection = selection;

            let head_row = self.table_header_row_tag_mut();
            if head_row.is_first_child_contains_class("mdc-data-table__header-cell--checkbox") {
                head_row.children.remove(0);
            }

            let body = self.table_body_tag_mut();
            for row in body.children.iter_mut() {
                if row.is_first_child_contains_class("mdc-data-table__cell--checkbox") {
                    if let Html::VTag(row) = row {
                        row.children.remove(0);
                    }
                }
            }
        }
        self
    }

    pub fn row_count(&self) -> usize {
        self.table_body_tag().children.len()
    }

    pub fn root_id(&self) -> &str {
        if let Html::VTag(tag) = &self.html {
            tag.attributes.get("id").expect("The DataTable widget must have ID")
        } else {
            panic!("The DataTable widget must be contains the root tag!")
        }
    }

    pub fn table_tag(&self) -> &VTag {
        if let Html::VTag(tag) = &self.html {
            if let Some(Html::VTag(tag)) = tag.children.first() {
                if let Some(table) = tag.first_child_tag("table") {
                    return table;
                }
            }
        }
        panic!("The DataTable widget must be contains the table tag!");
    }

    fn table_tag_mut(&mut self) -> &mut VTag {
        if let Html::VTag(tag) = &mut self.html {
            if let Some(Html::VTag(tag)) = tag.children.first_mut() {
                if let Some(table) = tag.first_child_tag_mut("table") {
                    return table;
                }
            }
        }
        panic!("The DataTable widget must be contains the table tag!");
    }

    fn table_header_row_tag_mut(&mut self) -> &mut VTag {
        if let Some(head) = self.table_tag_mut().first_child_tag_mut("thead") {
            if let Some(row) = head.first_child_tag_mut("tr") {
                return row;
            }
        }
        panic!("The DataTable widget must be contains the table header row tag!");
    }

    pub fn table_body_tag(&self) -> &VTag {
        match self.table_tag().children.get(1) {
            Some(Html::VTag(tag)) if tag.tag() == "tbody" => tag,
            _ => panic!("The DataTable widget must be contains the table body tag!"),
        }
    }

    fn table_body_tag_mut(&mut self) -> &mut VTag {
        match self.table_tag_mut().children.get_mut(1) {
            Some(Html::VTag(tag)) if tag.tag() == "tbody" => tag,
            _ => panic!("The DataTable widget must be contains the table body tag!"),
        }
    }

    fn add_on_click_to_row(row: &mut Html, on_row_click: OnRowClickFn) {
        if let Html::VTag(tag) = row {
            let callback = on_row_click(tag);
            tag.add_listener(Rc::new(onclick::Wrapper::new(callback)))
        }
    }

    fn head_checkbox(root_id: impl AsRef<str>) -> Html {
        let root_id = root_id.as_ref();
        let checkbox = Checkbox::new(format!("{}-header-checkbox", root_id))
            .class("mdc-data-table__header-row-checkbox mdc-checkbox--selected");
        html! {
            <th class = "mdc-data-table__header-cell mdc-data-table__header-cell--checkbox" role = "columnheader" scope = "col">
                { checkbox }
            </th>
        }
    }

    fn row_checkbox(row_id: impl AsRef<str>) -> Html {
        let row_id = row_id.as_ref();
        let checkbox = Checkbox::new(format!("{}-checkbox", row_id))
            .class("mdc-data-table__row-checkbox")
            .labeled_by(row_id);
        html! {
            <td class = "mdc-data-table__cell mdc-data-table__cell--checkbox">
                { checkbox }
            </td>
        }
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
    fn from(table: DataTable) -> Self {
        table.html
    }
}
