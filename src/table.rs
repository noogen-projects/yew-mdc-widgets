use std::{
    ops::{Deref, DerefMut},
    slice::{Iter, IterMut},
    vec::IntoIter,
};

use yew::{html, Callback, Html, MouseEvent};

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
    type IntoIter = IntoIter<TableCell<'a>>;
    type Item = TableCell<'a>;

    #[inline]
    fn into_iter(self) -> IntoIter<TableCell<'a>> {
        self.0.into_iter()
    }
}

impl<'a, 'b> IntoIterator for &'a TableRow<'b> {
    type IntoIter = Iter<'a, TableCell<'b>>;
    type Item = &'a TableCell<'b>;

    fn into_iter(self) -> Iter<'a, TableCell<'b>> {
        self.0.iter()
    }
}

impl<'a, 'b> IntoIterator for &'a mut TableRow<'b> {
    type IntoIter = IterMut<'a, TableCell<'b>>;
    type Item = &'a mut TableCell<'b>;

    fn into_iter(self) -> IterMut<'a, TableCell<'b>> {
        self.0.iter_mut()
    }
}

pub type ClickOnRowFn = Option<fn(&TableRow) -> Callback<MouseEvent>>;

pub fn table(
    id: impl AsRef<str>, caption: impl AsRef<str>, head: &[TableCell], body: &[TableRow],
    click_on_row: Option<impl Fn(&TableRow) -> Callback<MouseEvent>>,
) -> Html {
    let id = id.as_ref();
    let caption = caption.as_ref();

    html! {
        <div class = "user-table mdc-data-table">
            <table id = id class = "mdc-data-table__table" aria-label = caption>
                <thead>
                    <tr class = "mdc-data-table__header-row">
                        { head.into_iter().map(|cell| view_head_cell(cell)).collect::<Html>() }
                    </tr>
                </thead>
                <tbody class = "mdc-data-table__content">{
                    body
                        .into_iter()
                        .map(|row| {
                            let onclick = click_on_row.as_ref().map(|fun| fun(&row));
                            let cells = row.into_iter().map(|cell| view_body_cell(cell)).collect::<Html>();

                            if let Some(onclick) = onclick {
                                html! {
                                    <tr class = "mdc-data-table__row" onclick = onclick>{ cells }</tr>
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
