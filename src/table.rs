use std::{
    ops::{Deref, DerefMut},
    slice::{Iter, IterMut},
    vec::IntoIter,
};

use yew::{html, Callback, Html, MouseEvent};

use crate::{Text, Widget};

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

pub struct TableRow<'a>(Vec<TableCell<'a>>);

impl<'a> TableRow<'a> {
    pub fn new(row: Vec<TableCell<'a>>) -> Self {
        Self(row)
    }
}

impl<'a> Deref for TableRow<'a> {
    type Target = Vec<TableCell<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TableRow<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> IntoIterator for TableRow<'a> {
    type Item = TableCell<'a>;
    type IntoIter = IntoIter<TableCell<'a>>;

    #[inline]
    fn into_iter(self) -> IntoIter<TableCell<'a>> {
        self.0.into_iter()
    }
}

impl<'a, 'b> IntoIterator for &'a TableRow<'b> {
    type Item = &'a TableCell<'b>;
    type IntoIter = Iter<'a, TableCell<'b>>;

    fn into_iter(self) -> Iter<'a, TableCell<'b>> {
        self.0.iter()
    }
}

impl<'a, 'b> IntoIterator for &'a mut TableRow<'b> {
    type Item = &'a mut TableCell<'b>;
    type IntoIter = IterMut<'a, TableCell<'b>>;

    fn into_iter(self) -> IterMut<'a, TableCell<'b>> {
        self.0.iter_mut()
    }
}

pub type OnRowClickFn = fn(&TableRow) -> Callback<MouseEvent>;

pub struct Table<'a> {
    id: Text<'a>,
    caption: Text<'a>,
    head: &'a [TableCell<'a>],
    body: &'a [TableRow<'a>],
    on_row_click: Option<OnRowClickFn>,
}

impl<'a> Table<'a> {
    pub fn new() -> Self {
        Self {
            id: "".into(),
            caption: "".into(),
            head: &[],
            body: &[],
            on_row_click: None,
        }
    }

    pub fn id(mut self, id: impl Into<Text<'a>>) -> Self {
        self.id = id.into();
        self
    }

    pub fn caption(mut self, caption: impl Into<Text<'a>>) -> Self {
        self.caption = caption.into();
        self
    }

    pub fn head(mut self, head: &'a [TableCell<'a>]) -> Self {
        self.head = head;
        self
    }

    pub fn body(mut self, body: &'a [TableRow<'a>]) -> Self {
        self.body = body;
        self
    }

    pub fn on_row_click(mut self, on_row_click: OnRowClickFn) -> Self {
        self.on_row_click = Some(on_row_click);
        self
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

impl Widget for Table<'_> {
    fn build(&self) -> Html {
        html! {
            <div class = "user-table mdc-data-table">
                <table id = self.id class = "mdc-data-table__table" aria-label = self.caption>
                    <thead>
                        <tr class = "mdc-data-table__header-row">
                            { self.head.into_iter().map(|cell| Self::view_head_cell(cell)).collect::<Html>() }
                        </tr>
                    </thead>
                    <tbody class = "mdc-data-table__content">{
                        self.body
                            .into_iter()
                            .map(|row| {
                                let cells = row.into_iter().map(|cell| Self::view_body_cell(cell)).collect::<Html>();

                                if let Some(callback) = self.on_row_click.as_ref().map(|fun| fun(&row)) {
                                    html! {
                                        <tr class = "mdc-data-table__row" onclick = callback>{ cells }</tr>
                                    }
                                } else {
                                    html! {
                                        <tr class = "mdc-data-table__row">{ cells }</tr>
                                    }
                                }
                            })
                            .collect::<Html>()
                    }</tbody>
                </table>
            </div>
        }
    }
}
