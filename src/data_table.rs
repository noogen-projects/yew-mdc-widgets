use std::{
    rc::Rc,
    ops::{Deref, DerefMut},
};

use yew::{html, html::onclick, Callback, Html, MouseEvent, virtual_dom::VTag};

use crate::Text;

pub enum TableCell<'a> {
    Numeric(Text<'a>),
    Text(Text<'a>),
}

impl<'a> TableCell<'a> {
    pub fn num(content: impl Into<Text<'a>>) -> Self {
        TableCell::Numeric(content.into())
    }

    pub fn text(content: impl Into<Text<'a>>) -> Self {
        TableCell::Text(content.into())
    }

    pub fn content(&self) -> &Text<'a> {
        match self {
            TableCell::Numeric(content) => content,
            TableCell::Text(content) => content,
        }
    }
}

pub type OnRowClickFn = fn(&VTag) -> Callback<MouseEvent>;

pub struct DataTable {
    html: Html,
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
                            </thead>
                            <tbody class = "mdc-data-table__content">
                            </tbody>
                        </table>
                    </div>
                    <script>{ init_table }</script>
                </div>
            },
            on_row_click: None,
        }
    }

    pub fn head<'a>(mut self, head: &'a [TableCell<'a>]) -> Self {
        let head: Html = head
            .into_iter()
            .map(|cell| Self::view_head_cell(cell))
            .collect();

        let head_tag = self.table_head_tag_mut();
        head_tag.children.insert(0, html! {
            <tr class = "mdc-data-table__header-row">{ head }</tr>
        });
        self
    }

    pub fn row<'a>(mut self, row: &'a [TableCell<'a>]) -> Self {
        let row: Html = row
            .into_iter()
            .map(|cell| Self::view_body_cell(cell))
            .collect();
        let mut row = html! {
            <tr class = "mdc-data-table__row">{ row }</tr>
        };

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

    pub fn table_tag(&self) -> &VTag {
        if let Html::VTag(tag) = &self.html {
            if let Some(Html::VTag(tag)) = tag.children.first() {
                match tag.children.first() {
                    Some(Html::VTag(tag)) if tag.tag() == "table" => return tag,
                    _ => (),
                }
            }
        }
        panic!("The DataTable widget must be contains the table tag!");
    }

    pub fn table_tag_mut(&mut self) -> &mut VTag {
        if let Html::VTag(tag) = &mut self.html {
            if let Some(Html::VTag(tag)) = tag.children.first_mut() {
                match tag.children.first_mut() {
                    Some(Html::VTag(tag)) if tag.tag() == "table" => return tag,
                    _ => (),
                }
            }
        }
        panic!("The DataTable widget must be contains the table tag!");
    }

    pub fn table_head_tag_mut(&mut self) -> &mut VTag {
        match self.table_tag_mut().children.first_mut() {
            Some(Html::VTag(tag)) if tag.tag() == "thead" => {
                tag
            },
            _ => panic!("The DataTable widget must be contains the table head tag!"),
        }
    }

    pub fn table_body_tag_mut(&mut self) -> &mut VTag {
        match self.table_tag_mut().children.get_mut(1) {
            Some(Html::VTag(tag)) if tag.tag() == "tbody" => {
                tag
            },
            _ => panic!("The DataTable widget must be contains the table body tag!"),
        }
    }

    fn add_on_click_to_row(row: &mut Html, on_row_click: OnRowClickFn) {
        if let Html::VTag(tag) = row {
            let callback = on_row_click(tag);
            tag.add_listener(Rc::new(onclick::Wrapper::new(callback)))
        }
    }

    fn view_head_cell(cell: &TableCell) -> Html {
        let (class, content) = match cell {
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

    fn view_body_cell(cell: &TableCell) -> Html {
        let (class, content) = match cell {
            TableCell::Numeric(content) => ("mdc-data-table__cell mdc-data-table__cell--numeric", content),
            TableCell::Text(content) => ("mdc-data-table__cell", content),
        };
        html! {
            <td class = class>{ content }</td>
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