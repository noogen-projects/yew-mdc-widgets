use std::ops::{Deref, DerefMut};

use yew::{
    virtual_dom::{AttrValue, Attributes, VList, VNode, VTag},
    Html,
};

pub trait ManageChildren {
    fn is_first_child_contains_class(&self, class: &str) -> bool;
    fn is_some_child_contains_class(&self, class: &str) -> bool;
    fn find_child_contains_class_idx(&self, class: &str) -> Option<usize>;
    fn find_child_contains_class_mut(&mut self, class: &str) -> Option<&mut VTag>;
    fn find_child_contains_class_recursively_mut(&mut self, class: &str) -> Option<&mut VTag>;
    fn remove_child_contains_class(&mut self, class: &str) -> Option<Html>;
    fn is_first_child(&self, child_tag_name: &str) -> bool;
    fn is_last_child(&self, child_tag_name: &str) -> bool;
    fn first_child(&self) -> Option<&Html>;
    fn first_child_mut(&mut self) -> Option<&mut Html>;
    fn first_child_tag(&self) -> Option<&VTag>;
    fn first_child_tag_mut(&mut self) -> Option<&mut VTag>;
    fn last_child(&self) -> Option<&Html>;
    fn last_child_mut(&mut self) -> Option<&mut Html>;
    fn last_child_tag(&self) -> Option<&VTag>;
    fn last_child_tag_mut(&mut self) -> Option<&mut VTag>;
    fn find_child_tag(&self, child_tag_name: &str) -> Option<&VTag>;
    fn find_child_tag_mut(&mut self, child_tag_name: &str) -> Option<&mut VTag>;
    fn find_child_tag_idx(&self, child_tag_name: &str) -> Option<usize>;
    fn find_child_tag_recursively(&self, child_tag_name: &str) -> Option<&VTag>;
    fn find_child_tag_recursively_mut(&mut self, child_tag_name: &str) -> Option<&mut VTag>;
    fn remove_child_tag(&mut self, child_tag_name: &str) -> Option<Html>;
    fn add_child(&mut self, child: impl Into<Html>);
    fn add_child_script_statement(&mut self, statement: impl AsRef<str>);
    fn insert_child(&mut self, idx: usize, child: impl Into<Html>);
    fn remove_child(&mut self, idx: usize) -> Option<Html>;
    fn clear_children(&mut self);
}

pub trait VTagExt: ManageChildren {
    fn root_tag(&self) -> Option<&VTag>;
    fn root_tag_mut(&mut self) -> Option<&mut VTag>;
    fn add_class(&mut self, class: impl AsRef<str>);
    fn add_class_if_needed(&mut self, class: impl AsRef<str>);
    fn remove_class(&mut self, class: &str);
    fn remove_any_class(&mut self, classes: &[&str]);
    fn attr(&self, attr: impl AsRef<str>) -> Option<AttrValue>;
    fn set_attr(&mut self, attr: &'static str, value: impl Into<AttrValue>);
    fn remove_attr(&mut self, attr: impl AsRef<str>) -> Option<AttrValue>;
    fn is_contains_class(&self, class: &str) -> bool;
    fn is_contains_any_class(&self, classes: &[&str]) -> bool;
}

impl ManageChildren for VTag {
    fn is_first_child_contains_class(&self, class: &str) -> bool {
        if let Some(child) = self.first_child_tag() {
            child.is_contains_class(class)
        } else {
            false
        }
    }

    fn is_some_child_contains_class(&self, class: &str) -> bool {
        is_some_child_contains_class(self.children().iter(), class)
    }

    fn find_child_contains_class_idx(&self, class: &str) -> Option<usize> {
        find_child_contains_class_idx(self.children().iter(), class)
    }

    fn find_child_contains_class_mut(&mut self, class: &str) -> Option<&mut VTag> {
        self.children_mut()
            .and_then(|children| find_child_contains_class_mut(children.iter_mut(), class))
    }

    fn find_child_contains_class_recursively_mut(&mut self, class: &str) -> Option<&mut VTag> {
        self.children_mut()
            .and_then(|children| find_child_contains_class_recursively_mut(children.iter_mut(), class))
    }

    fn remove_child_contains_class(&mut self, class: &str) -> Option<Html> {
        self.find_child_contains_class_idx(class)
            .and_then(|idx| self.remove_child(idx))
    }

    fn is_first_child(&self, child_tag_name: &str) -> bool {
        get_tag(self.first_child(), child_tag_name).is_some()
    }

    fn is_last_child(&self, child_tag_name: &str) -> bool {
        get_tag(self.last_child(), child_tag_name).is_some()
    }

    fn first_child(&self) -> Option<&Html> {
        self.children().first()
    }

    fn first_child_mut(&mut self) -> Option<&mut Html> {
        self.children_mut().and_then(|children| children.first_mut())
    }

    fn first_child_tag(&self) -> Option<&VTag> {
        if let Html::VTag(tag) = self.first_child()? {
            Some(tag)
        } else {
            None
        }
    }

    fn first_child_tag_mut(&mut self) -> Option<&mut VTag> {
        if let Html::VTag(tag) = self.first_child_mut()? {
            Some(tag)
        } else {
            None
        }
    }

    fn last_child(&self) -> Option<&Html> {
        self.children().last()
    }

    fn last_child_mut(&mut self) -> Option<&mut Html> {
        self.children_mut().and_then(|children| children.last_mut())
    }

    fn last_child_tag(&self) -> Option<&VTag> {
        if let Html::VTag(tag) = self.last_child()? {
            Some(tag)
        } else {
            None
        }
    }

    fn last_child_tag_mut(&mut self) -> Option<&mut VTag> {
        if let Html::VTag(tag) = self.last_child_mut()? {
            Some(tag)
        } else {
            None
        }
    }

    fn find_child_tag(&self, child_tag_name: &str) -> Option<&VTag> {
        find_child_tag(self.children().iter(), child_tag_name)
    }

    fn find_child_tag_mut(&mut self, child_tag_name: &str) -> Option<&mut VTag> {
        self.children_mut()
            .and_then(|children| find_child_tag_mut(children.iter_mut(), child_tag_name))
    }

    fn find_child_tag_idx(&self, child_tag_name: &str) -> Option<usize> {
        find_child_tag_idx(self.children().iter(), child_tag_name)
    }

    fn find_child_tag_recursively(&self, child_tag_name: &str) -> Option<&VTag> {
        find_child_tag_recursively(self.children().iter(), child_tag_name)
    }

    fn find_child_tag_recursively_mut(&mut self, child_tag_name: &str) -> Option<&mut VTag> {
        self.children_mut()
            .and_then(|children| find_child_tag_recursively_mut(children.iter_mut(), child_tag_name))
    }

    fn remove_child_tag(&mut self, child_tag_name: &str) -> Option<Html> {
        self.find_child_tag_idx(child_tag_name)
            .and_then(|idx| self.remove_child(idx))
    }

    fn add_child(&mut self, child: impl Into<Html>) {
        if let Some(children) = self.children_mut() {
            children.push(child.into());
        }
    }

    fn add_child_script_statement(&mut self, statement: impl AsRef<str>) {
        add_child_script_statement(self.find_child_tag_mut("script"), statement)
    }

    fn insert_child(&mut self, idx: usize, child: impl Into<Html>) {
        if let Some(children) = self.children_mut() {
            children.insert(idx, child.into());
        }
    }

    fn remove_child(&mut self, idx: usize) -> Option<Html> {
        self.children_mut().map(|children| children.remove(idx))
    }

    fn clear_children(&mut self) {
        if let Some(children) = self.children_mut() {
            children.clear();
        }
    }
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
        if let Some(classes) = self.attr("class") {
            let classes = format!("{} {}", classes, class);
            self.set_attr("class", classes);
        } else {
            self.set_attr("class", class.to_string());
        }
    }

    fn add_class_if_needed(&mut self, class: impl AsRef<str>) {
        let class = class.as_ref().trim();
        if !self.is_contains_class(class) {
            self.add_class(class);
        }
    }

    fn remove_class(&mut self, class: &str) {
        if let Some(classes) = self.attr("class") {
            let classes = classes
                .split_whitespace()
                .filter(|item| class != *item)
                .collect::<Vec<_>>()
                .join(" ");
            self.set_attr("class", classes);
        }
    }

    fn remove_any_class(&mut self, classes: &[&str]) {
        if let Some(class) = self.attr("class") {
            let class = class
                .split_whitespace()
                .filter(|item| !classes.contains(item))
                .collect::<Vec<_>>()
                .join(" ");
            self.set_attr("class", class);
        }
    }

    fn attr(&self, attr: impl AsRef<str>) -> Option<AttrValue> {
        match &self.attributes {
            Attributes::Static(attrs) => attrs.iter().find_map(|[key, value]| {
                if *key == attr.as_ref() {
                    Some((*value).into())
                } else {
                    None
                }
            }),
            Attributes::Dynamic { keys, values } => {
                keys.iter()
                    .zip(values.iter())
                    .find_map(|(key, value)| if *key == attr.as_ref() { value.clone() } else { None })
            },
            Attributes::IndexMap(attrs) => attrs.get(attr.as_ref()).cloned(),
        }
    }

    fn set_attr(&mut self, attr: &'static str, value: impl Into<AttrValue>) {
        self.attributes.get_mut_index_map().insert(attr, value.into());
    }

    fn remove_attr(&mut self, attr: impl AsRef<str>) -> Option<AttrValue> {
        self.attributes.get_mut_index_map().remove(attr.as_ref())
    }

    fn is_contains_class(&self, class: &str) -> bool {
        if let Some(classes) = self.attr("class") {
            classes.split_whitespace().any(|item| item == class)
        } else {
            false
        }
    }

    fn is_contains_any_class(&self, classes: &[&str]) -> bool {
        if let Some(class) = self.attr("class") {
            class.split_whitespace().any(|item| classes.contains(&item))
        } else {
            false
        }
    }
}

impl ManageChildren for VList {
    fn is_first_child_contains_class(&self, class: &str) -> bool {
        if let Some(Html::VTag(child)) = self.first() {
            child.is_contains_class(class)
        } else {
            false
        }
    }

    fn is_some_child_contains_class(&self, class: &str) -> bool {
        is_some_child_contains_class(self.iter(), class)
    }

    fn find_child_contains_class_idx(&self, class: &str) -> Option<usize> {
        find_child_contains_class_idx(self.iter(), class)
    }

    fn find_child_contains_class_mut(&mut self, class: &str) -> Option<&mut VTag> {
        find_child_contains_class_mut(self.iter_mut(), class)
    }

    fn find_child_contains_class_recursively_mut(&mut self, class: &str) -> Option<&mut VTag> {
        find_child_contains_class_recursively_mut(self.iter_mut(), class)
    }

    fn remove_child_contains_class(&mut self, class: &str) -> Option<Html> {
        find_child_contains_class_idx(self.iter(), class).map(|idx| self.remove(idx))
    }

    fn is_first_child(&self, child_tag_name: &str) -> bool {
        get_tag(self.first(), child_tag_name).is_some()
    }

    fn is_last_child(&self, child_tag_name: &str) -> bool {
        get_tag(self.last(), child_tag_name).is_some()
    }

    fn first_child(&self) -> Option<&Html> {
        self.first()
    }

    fn first_child_mut(&mut self) -> Option<&mut Html> {
        self.first_mut()
    }

    fn first_child_tag(&self) -> Option<&VTag> {
        if let Html::VTag(tag) = self.first_child()? {
            Some(tag)
        } else {
            None
        }
    }

    fn first_child_tag_mut(&mut self) -> Option<&mut VTag> {
        if let Html::VTag(tag) = self.first_child_mut()? {
            Some(tag)
        } else {
            None
        }
    }

    fn last_child(&self) -> Option<&Html> {
        self.last()
    }

    fn last_child_mut(&mut self) -> Option<&mut Html> {
        self.last_mut()
    }

    fn last_child_tag(&self) -> Option<&VTag> {
        if let Html::VTag(tag) = self.last_child()? {
            Some(tag)
        } else {
            None
        }
    }

    fn last_child_tag_mut(&mut self) -> Option<&mut VTag> {
        if let Html::VTag(tag) = self.last_child_mut()? {
            Some(tag)
        } else {
            None
        }
    }

    fn find_child_tag(&self, child_tag_name: &str) -> Option<&VTag> {
        find_child_tag(self.iter(), child_tag_name)
    }

    fn find_child_tag_mut(&mut self, child_tag_name: &str) -> Option<&mut VTag> {
        find_child_tag_mut(self.iter_mut(), child_tag_name)
    }

    fn find_child_tag_idx(&self, child_tag_name: &str) -> Option<usize> {
        find_child_tag_idx(self.iter(), child_tag_name)
    }

    fn find_child_tag_recursively(&self, child_tag_name: &str) -> Option<&VTag> {
        find_child_tag_recursively(self.iter(), child_tag_name)
    }

    fn find_child_tag_recursively_mut(&mut self, child_tag_name: &str) -> Option<&mut VTag> {
        find_child_tag_recursively_mut(self.iter_mut(), child_tag_name)
    }

    fn remove_child_tag(&mut self, child_tag_name: &str) -> Option<Html> {
        find_child_tag_idx(self.iter(), child_tag_name).map(|idx| self.remove(idx))
    }

    fn add_child(&mut self, child: impl Into<Html>) {
        self.push(child.into())
    }

    fn add_child_script_statement(&mut self, statement: impl AsRef<str>) {
        add_child_script_statement(find_child_tag_mut(self.iter_mut(), "script"), statement)
    }

    fn insert_child(&mut self, idx: usize, child: impl Into<Html>) {
        self.insert(idx, child.into())
    }

    fn remove_child(&mut self, idx: usize) -> Option<Html> {
        Some(self.remove(idx))
    }

    fn clear_children(&mut self) {
        self.clear()
    }
}

impl ManageChildren for Html {
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

    fn find_child_contains_class_recursively_mut(&mut self, class: &str) -> Option<&mut VTag> {
        match self {
            Html::VTag(tag) => tag.find_child_contains_class_recursively_mut(class),
            Html::VList(list) => find_child_contains_class_recursively_mut(list.iter_mut(), class),
            _ => None,
        }
    }

    fn remove_child_contains_class(&mut self, class: &str) -> Option<Html> {
        match self {
            Html::VTag(tag) => tag.remove_child_contains_class(class),
            Html::VList(list) => find_child_contains_class_idx(list.iter(), class).map(|idx| list.remove(idx)),
            _ => None,
        }
    }

    fn is_first_child(&self, child_tag_name: &str) -> bool {
        match self {
            Html::VTag(tag) => tag.is_first_child(child_tag_name),
            Html::VList(list) => get_tag(list.first(), child_tag_name).is_some(),
            _ => false,
        }
    }

    fn is_last_child(&self, child_tag_name: &str) -> bool {
        match self {
            Html::VTag(tag) => tag.is_last_child(child_tag_name),
            Html::VList(list) => get_tag(list.last(), child_tag_name).is_some(),
            _ => false,
        }
    }

    fn first_child(&self) -> Option<&Html> {
        match self {
            Html::VTag(tag) => tag.first_child(),
            Html::VList(list) => list.first_child(),
            _ => None,
        }
    }

    fn first_child_mut(&mut self) -> Option<&mut Html> {
        match self {
            Html::VTag(tag) => tag.first_child_mut(),
            Html::VList(list) => list.first_child_mut(),
            _ => None,
        }
    }

    fn first_child_tag(&self) -> Option<&VTag> {
        match self {
            Html::VTag(tag) => tag.first_child_tag(),
            Html::VList(list) => list.first_child_tag(),
            _ => None,
        }
    }

    fn first_child_tag_mut(&mut self) -> Option<&mut VTag> {
        match self {
            Html::VTag(tag) => tag.first_child_tag_mut(),
            Html::VList(list) => list.first_child_tag_mut(),
            _ => None,
        }
    }

    fn last_child(&self) -> Option<&Html> {
        match self {
            Html::VTag(tag) => tag.last_child(),
            Html::VList(list) => list.last_child(),
            _ => None,
        }
    }

    fn last_child_mut(&mut self) -> Option<&mut Html> {
        match self {
            Html::VTag(tag) => tag.last_child_mut(),
            Html::VList(list) => list.last_child_mut(),
            _ => None,
        }
    }

    fn last_child_tag(&self) -> Option<&VTag> {
        match self {
            Html::VTag(tag) => tag.last_child_tag(),
            Html::VList(list) => list.last_child_tag(),
            _ => None,
        }
    }

    fn last_child_tag_mut(&mut self) -> Option<&mut VTag> {
        match self {
            Html::VTag(tag) => tag.last_child_tag_mut(),
            Html::VList(list) => list.last_child_tag_mut(),
            _ => None,
        }
    }

    fn find_child_tag(&self, child_tag_name: &str) -> Option<&VTag> {
        match self {
            Html::VTag(tag) => tag.find_child_tag(child_tag_name),
            Html::VList(list) => find_child_tag(list.iter(), child_tag_name),
            _ => None,
        }
    }

    fn find_child_tag_mut(&mut self, child_tag_name: &str) -> Option<&mut VTag> {
        match self {
            Html::VTag(tag) => tag.find_child_tag_mut(child_tag_name),
            Html::VList(list) => find_child_tag_mut(list.iter_mut(), child_tag_name),
            _ => None,
        }
    }

    fn find_child_tag_idx(&self, child_tag_name: &str) -> Option<usize> {
        match self {
            Html::VTag(tag) => tag.find_child_tag_idx(child_tag_name),
            Html::VList(list) => find_child_tag_idx(list.iter(), child_tag_name),
            _ => None,
        }
    }

    fn find_child_tag_recursively(&self, child_tag_name: &str) -> Option<&VTag> {
        match self {
            Html::VTag(tag) => tag.find_child_tag_recursively(child_tag_name),
            Html::VList(list) => find_child_tag_recursively(list.iter(), child_tag_name),
            _ => None,
        }
    }

    fn find_child_tag_recursively_mut(&mut self, child_tag_name: &str) -> Option<&mut VTag> {
        match self {
            Html::VTag(tag) => tag.find_child_tag_recursively_mut(child_tag_name),
            Html::VList(list) => find_child_tag_recursively_mut(list.iter_mut(), child_tag_name),
            _ => None,
        }
    }

    fn remove_child_tag(&mut self, child_tag_name: &str) -> Option<Html> {
        match self {
            Html::VTag(tag) => tag.remove_child_tag(child_tag_name),
            Html::VList(list) => find_child_tag_idx(list.iter(), child_tag_name).map(|idx| list.remove(idx)),
            _ => None,
        }
    }

    fn add_child(&mut self, child: impl Into<Html>) {
        match self {
            VNode::VTag(tag) => tag.add_child(child.into()),
            VNode::VList(list) => list.push(child.into()),
            _ => (),
        }
    }

    fn add_child_script_statement(&mut self, statement: impl AsRef<str>) {
        match self {
            Html::VTag(tag) => tag.add_child_script_statement(statement),
            Html::VList(list) => add_child_script_statement(find_child_tag_mut(list.iter_mut(), "script"), statement),
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

    fn remove_child(&mut self, idx: usize) -> Option<Html> {
        match self {
            Html::VTag(tag) => tag.remove_child(idx),
            Html::VList(list) => list.remove_child(idx),
            _ => None,
        }
    }

    fn clear_children(&mut self) {
        match self {
            Html::VTag(tag) => tag.clear_children(),
            Html::VList(list) => list.clear_children(),
            _ => (),
        }
    }
}

impl VTagExt for Html {
    fn root_tag(&self) -> Option<&VTag> {
        match self {
            Html::VTag(tag) => return Some(tag),
            Html::VList(list) => {
                if let Some(Html::VTag(tag)) = list.first() {
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
                if let Some(Html::VTag(tag)) = list.first_mut() {
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

    fn add_class_if_needed(&mut self, class: impl AsRef<str>) {
        if let Html::VTag(tag) = self {
            tag.add_class_if_needed(class);
        }
    }

    fn remove_class(&mut self, class: &str) {
        if let Html::VTag(tag) = self {
            tag.remove_class(class);
        }
    }

    fn remove_any_class(&mut self, classes: &[&str]) {
        if let Html::VTag(tag) = self {
            tag.remove_any_class(classes);
        }
    }

    fn attr(&self, attr: impl AsRef<str>) -> Option<AttrValue> {
        if let Html::VTag(tag) = self {
            tag.attr(attr)
        } else {
            None
        }
    }

    fn set_attr(&mut self, attr: &'static str, value: impl Into<AttrValue>) {
        if let Html::VTag(tag) = self {
            tag.set_attr(attr, value);
        }
    }

    fn remove_attr(&mut self, attr: impl AsRef<str>) -> Option<AttrValue> {
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
    children: impl IntoIterator<Item = &'a mut Html>,
    class: &str,
) -> Option<&'a mut VTag> {
    children.into_iter().find_map(|child| match child {
        Html::VTag(child) if child.is_contains_class(class) => Some(child.deref_mut()),
        _ => None,
    })
}

fn find_child_contains_class_recursively_mut<'a>(
    children: impl IntoIterator<Item = &'a mut Html>,
    class: &str,
) -> Option<&'a mut VTag> {
    for child in children {
        let tag = match child {
            Html::VTag(child) => {
                if child.is_contains_class(class) {
                    Some(child.deref_mut())
                } else {
                    child
                        .children_mut()
                        .and_then(|children| find_child_contains_class_recursively_mut(children.iter_mut(), class))
                }
            },
            Html::VList(list) => find_child_contains_class_recursively_mut(list.iter_mut(), class),
            _ => None,
        };
        if tag.is_some() {
            return tag;
        }
    }
    None
}

fn get_tag<'a>(html: Option<&'a Html>, tag_name: &str) -> Option<&'a VTag> {
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
    children: impl IntoIterator<Item = &'a mut Html>,
    child_tag_name: &str,
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
    children: impl IntoIterator<Item = &'a Html>,
    child_tag_name: &str,
) -> Option<&'a VTag> {
    for child in children {
        let tag = match child {
            Html::VTag(child) if child.tag() == child_tag_name => Some(child.deref()),
            Html::VTag(child) => find_child_tag_recursively(child.children().iter(), child_tag_name),
            Html::VList(list) => find_child_tag_recursively(list.iter(), child_tag_name),
            _ => None,
        };
        if tag.is_some() {
            return tag;
        }
    }
    None
}

fn find_child_tag_recursively_mut<'a>(
    children: impl IntoIterator<Item = &'a mut Html>,
    child_tag_name: &str,
) -> Option<&'a mut VTag> {
    for child in children {
        let tag = match child {
            Html::VTag(child) => {
                if child.tag() == child_tag_name {
                    Some(child.deref_mut())
                } else {
                    child
                        .children_mut()
                        .and_then(|children| find_child_tag_recursively_mut(children.iter_mut(), child_tag_name))
                }
            },
            Html::VList(list) => find_child_tag_recursively_mut(list.iter_mut(), child_tag_name),
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
        if let Some(Html::VText(text)) = script.first_child_mut() {
            let mut text_string = text.text.to_string();
            let insert_pos = if text_string.starts_with('{') && text_string.ends_with('}') {
                text_string.len() - 1
            } else if text_string.starts_with("setTimeout(") && text_string.ends_with("}, 0)") {
                text_string.len() - 5
            } else {
                text_string.len()
            };
            text_string.insert_str(insert_pos, statement.as_ref());
            text.text = text_string.into();
        }
    }
}
