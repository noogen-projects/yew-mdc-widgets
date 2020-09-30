use std::{
    borrow::Borrow,
    hash::Hash,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use yew::{
    html,
    virtual_dom::{Listener, VTag},
    Html,
};

pub trait VTagExt {
    fn root_tag(&self) -> Option<&VTag>;
    fn root_tag_mut(&mut self) -> Option<&mut VTag>;
    fn add_class(&mut self, class: impl AsRef<str>);
    fn remove_any_class(&mut self, classes: &[&str]);
    fn attr<Q>(&self, attr: &Q) -> Option<&String>
    where
        Q: ?Sized + Hash + Eq,
        String: Borrow<Q>;
    fn set_attr(&mut self, attr: impl Into<String>, value: impl Into<String>);
    fn remove_attr(&mut self, attr: impl AsRef<str>) -> Option<String>;
    fn is_contains_class(&self, class: &str) -> bool;
    fn is_contains_any_class(&self, classes: &[&str]) -> bool;
    fn is_first_child_contains_class(&self, class: &str) -> bool;
    fn is_some_child_contains_class(&self, class: &str) -> bool;
    fn find_child_contains_class_idx(&self, class: &str) -> Option<usize>;
    fn find_child_contains_class_mut(&mut self, class: &str) -> Option<&mut VTag>;
    fn remove_child_contains_class(&mut self, class: &str) -> Option<Html>;
    fn is_first_child(&self, child_tag_name: &str) -> bool;
    fn is_last_child(&self, child_tag_name: &str) -> bool;
    fn first_child_tag(&self, child_tag_name: &str) -> Option<&VTag>;
    fn first_child_tag_mut(&mut self, child_tag_name: &str) -> Option<&mut VTag>;
    fn find_child_tag(&self, child_tag_name: &str) -> Option<&VTag>;
    fn find_child_tag_mut(&mut self, child_tag_name: &str) -> Option<&mut VTag>;
    fn find_child_tag_idx(&self, child_tag_name: &str) -> Option<usize>;
    fn find_child_tag_recursively(&self, child_tag_name: &str) -> Option<&VTag>;
    fn find_child_tag_recursively_mut(&mut self, child_tag_name: &str) -> Option<&mut VTag>;
    fn remove_child_tag(&mut self, child_tag_name: &str) -> Option<Html>;
    fn add_child_script_statement(&mut self, statement: impl AsRef<str>);
    fn insert_child(&mut self, idx: usize, child: impl Into<Html>);
}

impl VTagExt for VTag {
    fn root_tag(&self) -> Option<&VTag> {
        Some(self)
    }

    fn root_tag_mut(&mut self) -> Option<&mut VTag> {
        Some(self)
    }

    fn add_class(&mut self, class: impl AsRef<str>) {
        let class = class.as_ref().trim();
        if let Some(classes) = self.attributes.get_mut("class") {
            classes.push(' ');
            classes.push_str(class)
        } else {
            self.attributes.insert("class".to_string(), class.to_string());
        }
    }

    fn remove_any_class(&mut self, classes: &[&str]) {
        if let Some(class) = self.attributes.get_mut("class") {
            *class = class
                .split_whitespace()
                .filter(|item| !classes.contains(item))
                .collect::<Vec<_>>()
                .join(" ");
        }
    }

    fn attr<Q>(&self, attr: &Q) -> Option<&String>
    where
        Q: ?Sized + Hash + Eq,
        String: Borrow<Q>,
    {
        self.attributes.get(attr)
    }

    fn set_attr(&mut self, attr: impl Into<String>, value: impl Into<String>) {
        self.attributes.insert(attr.into(), value.into());
    }

    fn remove_attr(&mut self, attr: impl AsRef<str>) -> Option<String> {
        self.attributes.remove(attr.as_ref())
    }

    fn is_contains_class(&self, class: &str) -> bool {
        if let Some(classes) = self.attributes.get("class") {
            classes.split_whitespace().any(|item| item == class)
        } else {
            false
        }
    }

    fn is_contains_any_class(&self, classes: &[&str]) -> bool {
        if let Some(class) = self.attributes.get("class") {
            class.split_whitespace().any(|item| classes.contains(&item))
        } else {
            false
        }
    }

    fn is_first_child_contains_class(&self, class: &str) -> bool {
        if let Some(Html::VTag(child)) = self.children.first() {
            child.is_contains_class(class)
        } else {
            false
        }
    }

    fn is_some_child_contains_class(&self, class: &str) -> bool {
        is_some_child_contains_class(self.children.iter(), class)
    }

    fn find_child_contains_class_idx(&self, class: &str) -> Option<usize> {
        find_child_contains_class_idx(self.children.iter(), class)
    }

    fn find_child_contains_class_mut(&mut self, class: &str) -> Option<&mut VTag> {
        find_child_contains_class_mut(self.children.iter_mut(), class)
    }

    fn remove_child_contains_class(&mut self, class: &str) -> Option<Html> {
        self.find_child_contains_class_idx(class)
            .map(|idx| self.children.remove(idx))
    }

    fn is_first_child(&self, child_tag_name: &str) -> bool {
        get_tag(self.children.first(), child_tag_name).is_some()
    }

    fn is_last_child(&self, child_tag_name: &str) -> bool {
        get_tag(self.children.last(), child_tag_name).is_some()
    }

    fn first_child_tag(&self, child_tag_name: &str) -> Option<&VTag> {
        get_tag(self.children.first(), child_tag_name)
    }

    fn first_child_tag_mut(&mut self, child_tag_name: &str) -> Option<&mut VTag> {
        get_tag_mut(self.children.first_mut(), child_tag_name)
    }

    fn find_child_tag(&self, child_tag_name: &str) -> Option<&VTag> {
        find_child_tag(self.children.iter(), child_tag_name)
    }

    fn find_child_tag_mut(&mut self, child_tag_name: &str) -> Option<&mut VTag> {
        find_child_tag_mut(self.children.iter_mut(), child_tag_name)
    }

    fn find_child_tag_idx(&self, child_tag_name: &str) -> Option<usize> {
        find_child_tag_idx(self.children.iter(), child_tag_name)
    }

    fn find_child_tag_recursively(&self, child_tag_name: &str) -> Option<&VTag> {
        find_child_tag_recursively(self.children.iter(), child_tag_name)
    }

    fn find_child_tag_recursively_mut(&mut self, child_tag_name: &str) -> Option<&mut VTag> {
        find_child_tag_recursively_mut(self.children.iter_mut(), child_tag_name)
    }

    fn remove_child_tag(&mut self, child_tag_name: &str) -> Option<Html> {
        self.find_child_tag_idx(child_tag_name)
            .map(|idx| self.children.remove(idx))
    }

    fn add_child_script_statement(&mut self, statement: impl AsRef<str>) {
        add_child_script_statement(self.find_child_tag_mut("script"), statement)
    }

    fn insert_child(&mut self, idx: usize, child: impl Into<Html>) {
        self.children.insert(idx, child.into());
    }
}

impl VTagExt for Html {
    fn root_tag(&self) -> Option<&VTag> {
        match self {
            Html::VTag(tag) => return Some(tag),
            Html::VList(list) => {
                if let Some(Html::VTag(tag)) = list.children.first() {
                    return Some(tag);
                }
            },
            _ => (),
        }
        None
    }

    fn root_tag_mut(&mut self) -> Option<&mut VTag> {
        match self {
            Html::VTag(tag) => return Some(tag),
            Html::VList(list) => {
                if let Some(Html::VTag(tag)) = list.children.first_mut() {
                    return Some(tag);
                }
            },
            _ => (),
        }
        None
    }

    fn add_class(&mut self, class: impl AsRef<str>) {
        if let Html::VTag(tag) = self {
            tag.add_class(class);
        }
    }

    fn remove_any_class(&mut self, classes: &[&str]) {
        if let Html::VTag(tag) = self {
            tag.remove_any_class(classes);
        }
    }

    fn attr<Q>(&self, attr: &Q) -> Option<&String>
    where
        Q: ?Sized + Hash + Eq,
        String: Borrow<Q>,
    {
        if let Html::VTag(tag) = self {
            tag.attr(attr)
        } else {
            None
        }
    }

    fn set_attr(&mut self, attr: impl Into<String>, value: impl Into<String>) {
        if let Html::VTag(tag) = self {
            tag.set_attr(attr, value);
        }
    }

    fn remove_attr(&mut self, attr: impl AsRef<str>) -> Option<String> {
        if let Html::VTag(tag) = self {
            tag.remove_attr(attr)
        } else {
            None
        }
    }

    fn is_contains_class(&self, class: &str) -> bool {
        if let Html::VTag(tag) = self {
            tag.is_contains_class(class)
        } else {
            false
        }
    }

    fn is_contains_any_class(&self, classes: &[&str]) -> bool {
        if let Html::VTag(tag) = self {
            tag.is_contains_any_class(classes)
        } else {
            false
        }
    }

    fn is_first_child_contains_class(&self, class: &str) -> bool {
        match self {
            Html::VTag(tag) => tag.is_first_child_contains_class(class),
            Html::VList(list) => {
                if let Some(Html::VTag(child)) = list.first() {
                    child.is_contains_class(class)
                } else {
                    false
                }
            },
            _ => false,
        }
    }

    fn is_some_child_contains_class(&self, class: &str) -> bool {
        match self {
            Html::VTag(tag) => tag.is_some_child_contains_class(class),
            Html::VList(list) => is_some_child_contains_class(list.iter(), class),
            _ => false,
        }
    }

    fn find_child_contains_class_idx(&self, class: &str) -> Option<usize> {
        match self {
            Html::VTag(tag) => tag.find_child_contains_class_idx(class),
            Html::VList(list) => find_child_contains_class_idx(list.iter(), class),
            _ => None,
        }
    }

    fn find_child_contains_class_mut(&mut self, class: &str) -> Option<&mut VTag> {
        match self {
            Html::VTag(tag) => tag.find_child_contains_class_mut(class),
            Html::VList(list) => find_child_contains_class_mut(list.iter_mut(), class),
            _ => None,
        }
    }

    fn remove_child_contains_class(&mut self, class: &str) -> Option<Html> {
        match self {
            Html::VTag(tag) => tag.remove_child_contains_class(class),
            Html::VList(list) => find_child_contains_class_idx(list.iter(), class).map(|idx| list.children.remove(idx)),
            _ => None,
        }
    }

    fn is_first_child(&self, child_tag_name: &str) -> bool {
        match self {
            Html::VTag(tag) => tag.is_first_child(child_tag_name),
            Html::VList(list) => get_tag(list.children.first(), child_tag_name).is_some(),
            _ => false,
        }
    }

    fn is_last_child(&self, child_tag_name: &str) -> bool {
        match self {
            Html::VTag(tag) => tag.is_last_child(child_tag_name),
            Html::VList(list) => get_tag(list.children.last(), child_tag_name).is_some(),
            _ => false,
        }
    }

    fn first_child_tag(&self, child_tag_name: &str) -> Option<&VTag> {
        match self {
            Html::VTag(tag) => tag.first_child_tag(child_tag_name),
            Html::VList(list) => get_tag(list.children.first(), child_tag_name),
            _ => None,
        }
    }

    fn first_child_tag_mut(&mut self, child_tag_name: &str) -> Option<&mut VTag> {
        match self {
            Html::VTag(tag) => tag.first_child_tag_mut(child_tag_name),
            Html::VList(list) => get_tag_mut(list.children.first_mut(), child_tag_name),
            _ => None,
        }
    }

    fn find_child_tag(&self, child_tag_name: &str) -> Option<&VTag> {
        match self {
            Html::VTag(tag) => tag.find_child_tag(child_tag_name),
            Html::VList(list) => find_child_tag(list.children.iter(), child_tag_name),
            _ => None,
        }
    }

    fn find_child_tag_mut(&mut self, child_tag_name: &str) -> Option<&mut VTag> {
        match self {
            Html::VTag(tag) => tag.find_child_tag_mut(child_tag_name),
            Html::VList(list) => find_child_tag_mut(list.children.iter_mut(), child_tag_name),
            _ => None,
        }
    }

    fn find_child_tag_idx(&self, child_tag_name: &str) -> Option<usize> {
        match self {
            Html::VTag(tag) => tag.find_child_tag_idx(child_tag_name),
            Html::VList(list) => find_child_tag_idx(list.children.iter(), child_tag_name),
            _ => None,
        }
    }

    fn find_child_tag_recursively(&self, child_tag_name: &str) -> Option<&VTag> {
        match self {
            Html::VTag(tag) => tag.find_child_tag_recursively(child_tag_name),
            Html::VList(list) => find_child_tag_recursively(list.children.iter(), child_tag_name),
            _ => None,
        }
    }

    fn find_child_tag_recursively_mut(&mut self, child_tag_name: &str) -> Option<&mut VTag> {
        match self {
            Html::VTag(tag) => tag.find_child_tag_recursively_mut(child_tag_name),
            Html::VList(list) => find_child_tag_recursively_mut(list.children.iter_mut(), child_tag_name),
            _ => None,
        }
    }

    fn remove_child_tag(&mut self, child_tag_name: &str) -> Option<Html> {
        match self {
            Html::VTag(tag) => tag.remove_child_tag(child_tag_name),
            Html::VList(list) => find_child_tag_idx(list.iter(), child_tag_name).map(|idx| list.children.remove(idx)),
            _ => None,
        }
    }

    fn add_child_script_statement(&mut self, statement: impl AsRef<str>) {
        match self {
            Html::VTag(tag) => tag.add_child_script_statement(statement),
            Html::VList(list) => {
                add_child_script_statement(find_child_tag_mut(list.children.iter_mut(), "script"), statement)
            },
            _ => (),
        }
    }

    fn insert_child(&mut self, idx: usize, child: impl Into<Html>) {
        match self {
            Html::VTag(tag) => tag.insert_child(idx, child),
            Html::VList(list) => list.insert(idx, child.into()),
            _ => (),
        }
    }
}

fn is_some_child_contains_class<'a>(children: impl IntoIterator<Item = &'a Html>, class: &str) -> bool {
    for child in children {
        if child.is_contains_class(class) {
            return true;
        }
    }
    false
}

fn find_child_contains_class_idx<'a>(children: impl IntoIterator<Item = &'a Html>, class: &str) -> Option<usize> {
    children.into_iter().enumerate().find_map(|(idx, child)| match child {
        Html::VTag(child) if child.is_contains_class(class) => Some(idx),
        _ => None,
    })
}

fn find_child_contains_class_mut<'a>(
    children: impl IntoIterator<Item = &'a mut Html>, class: &str,
) -> Option<&'a mut VTag> {
    children.into_iter().find_map(|child| match child {
        Html::VTag(child) if child.is_contains_class(class) => Some(child.deref_mut()),
        _ => None,
    })
}

fn get_tag<'a>(html: Option<&'a Html>, tag_name: &str) -> Option<&'a VTag> {
    match html {
        Some(Html::VTag(tag)) if tag.tag() == tag_name => Some(tag),
        _ => None,
    }
}

fn get_tag_mut<'a>(html: Option<&'a mut Html>, tag_name: &str) -> Option<&'a mut VTag> {
    match html {
        Some(Html::VTag(tag)) if tag.tag() == tag_name => Some(tag),
        _ => None,
    }
}

fn find_child_tag<'a>(children: impl IntoIterator<Item = &'a Html>, child_tag_name: &str) -> Option<&'a VTag> {
    for child in children {
        if let Html::VTag(child) = child {
            if child.tag() == child_tag_name {
                return Some(child);
            }
        }
    }
    None
}

fn find_child_tag_mut<'a>(
    children: impl IntoIterator<Item = &'a mut Html>, child_tag_name: &str,
) -> Option<&'a mut VTag> {
    for child in children {
        if let Html::VTag(child) = child {
            if child.tag() == child_tag_name {
                return Some(child);
            }
        }
    }
    None
}

fn find_child_tag_recursively<'a>(
    children: impl IntoIterator<Item = &'a Html>, child_tag_name: &str,
) -> Option<&'a VTag> {
    for child in children {
        let tag = match child {
            Html::VTag(child) if child.tag() == child_tag_name => Some(child.deref()),
            Html::VTag(child) => find_child_tag_recursively(child.children.iter(), child_tag_name),
            Html::VList(list) => find_child_tag_recursively(list.children.iter(), child_tag_name),
            _ => None,
        };
        if tag.is_some() {
            return tag;
        }
    }
    None
}

fn find_child_tag_recursively_mut<'a>(
    children: impl IntoIterator<Item = &'a mut Html>, child_tag_name: &str,
) -> Option<&'a mut VTag> {
    for child in children {
        let tag = match child {
            Html::VTag(child) => {
                if child.tag() == child_tag_name {
                    Some(child.deref_mut())
                } else {
                    find_child_tag_recursively_mut(child.children.iter_mut(), child_tag_name)
                }
            },
            Html::VList(list) => find_child_tag_recursively_mut(list.children.iter_mut(), child_tag_name),
            _ => None,
        };
        if tag.is_some() {
            return tag;
        }
    }
    None
}

fn find_child_tag_idx<'a>(children: impl IntoIterator<Item = &'a Html>, child_tag_name: &str) -> Option<usize> {
    for (idx, child) in children.into_iter().enumerate() {
        if let Html::VTag(child) = child {
            if child.tag() == child_tag_name {
                return Some(idx);
            }
        }
    }
    None
}

fn add_child_script_statement(child: Option<&mut VTag>, statement: impl AsRef<str>) {
    if let Some(script) = child {
        if let Some(Html::VText(text)) = script.children.children.first_mut() {
            let text = &mut text.text;
            if text.starts_with('{') && text.ends_with('}') {
                text.insert_str(text.len() - 1, statement.as_ref());
            } else {
                text.push_str(statement.as_ref());
            }
        }
    }
}

pub trait MdcWidget {
    const NAME: &'static str;

    fn html(&self) -> &Html;

    fn html_mut(&mut self) -> &mut Html;

    fn root_tag(&self) -> &VTag {
        self.html()
            .root_tag()
            .unwrap_or_else(|| panic!("The root element of the {} must be a tag!", Self::NAME))
    }

    fn root_tag_mut(&mut self) -> &mut VTag {
        self.html_mut()
            .root_tag_mut()
            .unwrap_or_else(|| panic!("The root element of the {} must be a tag!", Self::NAME))
    }

    fn listener(mut self, listener: Rc<dyn Listener>) -> Self
    where
        Self: Sized,
    {
        self.root_tag_mut().add_listener(listener);
        self
    }

    fn attr(mut self, attr: impl Into<String>, value: impl Into<String>) -> Self
    where
        Self: Sized,
    {
        self.root_tag_mut().set_attr(attr.into(), value.into());
        self
    }

    fn class(mut self, class: impl AsRef<str>) -> Self
    where
        Self: Sized,
    {
        self.root_tag_mut().add_class(class);
        self
    }
}

pub fn ripple(widget: &mut impl MdcWidget, ripple_class: impl AsRef<str>, enabled: bool) {
    let ripple_class = ripple_class.as_ref();
    let root = widget.root_tag_mut();
    if enabled {
        if !root.is_some_child_contains_class(ripple_class) {
            let idx = root.children.len().saturating_sub(1);
            root.children.insert(idx, html! {
                <div class = ripple_class></div>
            });
        }
    } else {
        root.remove_child_contains_class(ripple_class);
    }
}

pub fn root_and_input_child_disabled(widget: &mut impl MdcWidget, disabled_class: impl AsRef<str>, disabled: bool) {
    let disabled_class = disabled_class.as_ref();
    if disabled {
        widget.root_tag_mut().add_class(disabled_class);
    } else {
        widget.root_tag_mut().remove_any_class(&[disabled_class]);
    }

    if let Some(input) = widget.root_tag_mut().find_child_tag_mut("input") {
        if disabled {
            input.attributes.insert("disabled".into(), "disabled".into());
        } else {
            input.attributes.remove("disabled");
        }
    }
}
