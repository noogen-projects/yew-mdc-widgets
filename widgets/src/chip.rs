use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use yew::{html, html::onclick, Callback, Html, MouseEvent};

use crate::{
    utils::{MdcWidget, VTagExt},
    AUTO_INIT_ATTR,
};

#[derive(Debug, Clone)]
pub struct Chip {
    html: Html,
    tab_index: isize,
}

impl Chip {
    ///
    pub const CLASS: &'static str = "mdc-chip";

    /// Indicates the element which shows the ripple styling.
    pub const RIPPLE_CLASS: &'static str = "mdc-chip__ripple";

    /// See [`Chip::selected`] doc.
    pub const SELECTED_CLASS: &'static str = "mdc-chip--selected";

    ///
    pub const PRIMARY_ACTION_CLASS: &'static str = "mdc-chip__primary-action";

    /// Indicates the text content of the chip.
    pub const TEXT_CLASS: &'static str = "mdc-chip__text";

    /// Indicates an icon in the chip.
    pub const ICON_CLASS: &'static str = "mdc-chip__icon";

    /// Indicates a leading icon in the chip.
    pub const ICON_LEADING_CLASS: &'static str = "mdc-chip__icon--leading";

    /// Hides the leading icon in a filter chip when the chip is selected.
    pub const ICON_LEADING_HIDDEN_CLASS: &'static str = "mdc-chip__icon--leading-hidden";

    /// Indicates a trailing icon which removes the chip from the DOM. Only use with input chips.
    pub const ICON_TRAILING_CLASS: &'static str = "mdc-chip__icon--trailing";

    /// Indicates the checkmark in a filter chip.
    pub const CHECKMARK_CLASS: &'static str = "mdc-chip__checkmark";

    /// Mandatory with the use of [`Chip::CHECKMARK_CLASS`]. Indicates the checkmark SVG element in a filter chip.
    pub const CHECKMARK_SVG_CLASS: &'static str = "mdc-chip__checkmark-svg";

    /// Mandatory with the use of [`Chip::CHECKMARK_CLASS`]. Indicates the checkmark SVG path in a filter chip.
    pub const CHECKMARK_SVG_PATH_CLASS: &'static str = "mdc-chip__checkmark-path";

    pub fn simple() -> Self {
        Self {
            html: html! {
                <div class = Self::CLASS role = "row">
                    <div class = Self::RIPPLE_CLASS></div>
                </div>
            },
            tab_index: -1,
        }
    }

    pub fn new() -> Self {
        Self::simple().ripple(true)
    }

    /// Indicates that the chip is selected.
    pub fn selected(self) -> Self {
        self.select(true)
    }

    pub fn select(mut self, selected: bool) -> Self {
        let root = self.root_tag_mut();
        if let Some(icon) = root
            .find_child_contains_class_idx(Self::CHECKMARK_CLASS)
            .and_then(|_| root.find_child_contains_class_mut(Self::ICON_LEADING_CLASS))
        {
            if selected {
                icon.add_class_if_needed(Self::ICON_LEADING_HIDDEN_CLASS);
            } else {
                icon.remove_class(Self::ICON_LEADING_HIDDEN_CLASS);
            }
        }

        if selected {
            self.class(Self::SELECTED_CLASS)
        } else {
            self.remove_class(Self::SELECTED_CLASS);
            self
        }
    }

    pub fn ripple(mut self, enabled: bool) -> Self {
        let root = self.root_tag_mut();

        if enabled {
            root.set_attr(AUTO_INIT_ATTR, "MDCChip");
        } else {
            root.remove_attr(AUTO_INIT_ATTR);
        }
        self
    }

    pub fn icon(mut self, name: impl Into<String>) -> Self {
        let root = self.root_tag_mut();
        if root.children.len() < 2 {
            root.add_child(html! {
                <i class = vec!["material-icons", Self::ICON_CLASS, Self::ICON_LEADING_CLASS]>{ name.into() }</i>
            });
        } else {
            root.add_child(html! {
                <span role = "gridcell">
                    <i class = vec!["material-icons", Self::ICON_CLASS, Self::ICON_TRAILING_CLASS] tabindex = "-1" role = "button">{ name.into() }</i>
                </span>
            });
        }
        self
    }

    pub fn text(mut self, text: impl Into<Html>) -> Self {
        let tab_index = self.tab_index;
        self.root_tag_mut().add_child(html! {
            <span role = "gridcell">
                <span tabindex = tab_index class = Self::PRIMARY_ACTION_CLASS>
                    <span class = Self::TEXT_CLASS>{ text }</span>
                </span>
            </span>
        });
        self
    }

    pub fn tab_index(mut self, index: isize) -> Self {
        self.tab_index = index;
        if let Some(child) = self
            .root_tag_mut()
            .find_child_contains_class_recursively_mut(Self::PRIMARY_ACTION_CLASS)
        {
            child.set_attr("tabindex", format!("{}", index));
        }
        self
    }

    pub fn checkmark_svg(mut self, svg: impl Into<Html>) -> Self {
        let mut svg = svg.into();
        if let Some(root) = svg.root_tag_mut() {
            root.add_class_if_needed(Self::CHECKMARK_SVG_CLASS);
        }
        mark_svg_path(&mut svg);

        let root = self.root_tag_mut();
        root.add_child(html! {
            <span class = Self::CHECKMARK_CLASS>{ svg }</span>
        });

        if root.is_contains_class(Self::SELECTED_CLASS) {
            if let Some(icon) = root.find_child_contains_class_mut(Self::ICON_LEADING_CLASS) {
                icon.add_class_if_needed(Self::ICON_LEADING_HIDDEN_CLASS);
            }
        }
        self
    }

    pub fn checkmark(self) -> Self {
        self.checkmark_svg(html! {
            <svg viewBox = "-2 -3 30 30">
                <path fill = "none" stroke = "black"
                    d = "M1.73,12.91 8.1,19.28 22.79,4.59"/>
            </svg>
        })
    }

    pub fn tile(mut self, item: impl Into<Html>) -> Self {
        self.root_tag_mut().add_child(item.into());
        self
    }

    pub fn on_click(self, callback: Callback<MouseEvent>) -> Self {
        self.listener(Rc::new(onclick::Wrapper::new(callback)))
    }
}

fn mark_svg_path(parent: &mut Html) {
    match parent {
        Html::VTag(parent) => {
            if parent.tag() == "path" {
                parent.add_class_if_needed(Chip::CHECKMARK_SVG_PATH_CLASS);
            }
            for child in parent.children.iter_mut() {
                mark_svg_path(child);
            }
        }
        Html::VList(list) => {
            for item in list.iter_mut() {
                mark_svg_path(item);
            }
        }
        _ => (),
    }
}

impl MdcWidget for Chip {
    const NAME: &'static str = "Chip";

    fn html(&self) -> &Html {
        &self.html
    }

    fn html_mut(&mut self) -> &mut Html {
        &mut self.html
    }
}

impl Deref for Chip {
    type Target = Html;

    fn deref(&self) -> &Self::Target {
        &self.html
    }
}

impl DerefMut for Chip {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.html
    }
}

impl From<Chip> for Html {
    fn from(widget: Chip) -> Self {
        widget.html
    }
}

#[derive(Debug, Clone)]
pub struct ChipSet {
    html: Html,
}

impl ChipSet {
    /// See [`ChipSet::input`] doc.
    pub const INPUT_CLASS: &'static str = "mdc-chip-set--input";

    /// See [`ChipSet::choice`] doc.
    pub const CHOICE_CLASS: &'static str = "mdc-chip-set--choice";

    /// See [`ChipSet::filter`] doc.
    pub const FILTER_CLASS: &'static str = "mdc-chip-set--filter";

    pub fn simple() -> Self {
        Self {
            html: html! {
                <div class = "mdc-chip-set" role = "grid">
                </div>
            },
        }
    }

    pub fn new() -> Self {
        let mut chip_set = Self::simple();
        chip_set.root_tag_mut().set_attr(AUTO_INIT_ATTR, "MDCChipSet");
        chip_set
    }

    /// Indicates that the chips in the set are input chips, which enable user input by converting text into chips.
    pub fn input(self) -> Self {
        self.class(Self::INPUT_CLASS)
    }

    /// Indicates that the chips in the set are choice chips, which allow a single selection from a set of options.
    pub fn choice(self) -> Self {
        self.class(Self::CHOICE_CLASS)
    }

    /// Indicates that the chips in the set are filter chips, which allow multiple selection from a set of options.
    pub fn filter(self) -> Self {
        self.class(Self::FILTER_CLASS)
    }

    pub fn chip(mut self, chip: impl Into<Html>) -> Self {
        let mut chip = chip.into();
        let root = self.root_tag_mut();
        let chip_number = root.children.len();

        if chip.attr("id").is_none() && chip.is_some_child_contains_class(Chip::RIPPLE_CLASS) {
            if let (Some(id), Some(chip)) = (root.attr("id"), chip.root_tag_mut()) {
                chip.set_attr("id", format!("{}-chip-{}", id, chip_number));
            }
        }
        root.add_child(chip);
        self
    }

    pub fn chips(mut self, chips: impl IntoIterator<Item = impl Into<Html>>) -> Self {
        for chip in chips.into_iter() {
            self = self.chip(chip);
        }
        self
    }
}

impl MdcWidget for ChipSet {
    const NAME: &'static str = "ChipSet";

    fn html(&self) -> &Html {
        &self.html
    }

    fn html_mut(&mut self) -> &mut Html {
        &mut self.html
    }
}

impl Deref for ChipSet {
    type Target = Html;

    fn deref(&self) -> &Self::Target {
        &self.html
    }
}

impl DerefMut for ChipSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.html
    }
}

impl From<ChipSet> for Html {
    fn from(widget: ChipSet) -> Self {
        widget.html
    }
}
