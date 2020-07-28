use yew::{html, Html, virtual_dom::VTag};

pub trait VTagExt {
    fn is_contains_class(&self, class: &str) -> bool;
    fn is_contains_any_class(&self, classes: &[&str]) -> bool;
    fn add_class(&mut self, class: impl AsRef<str>);
    fn remove_any_class(&mut self, classes: &[&str]);
    fn is_first_child_contains_class(&self, class: &str) -> bool;
    fn is_some_child_contains_class(&self, class: &str) -> bool;
    fn find_child_contains_class(&self, class: &str) -> Option<usize>;
    fn is_first_child(&self, child_tag_name: &str) -> bool;
    fn is_last_child(&self, child_tag_name: &str) -> bool;
    fn first_child_tag(&self, child_tag_name: &str) -> Option<&VTag>;
    fn first_child_tag_mut(&mut self, child_tag_name: &str) -> Option<&mut VTag>;
    fn find_child_tag(&self, child_tag_name: &str) -> Option<&VTag>;
    fn find_child_tag_mut(&mut self, child_tag_name: &str) -> Option<&mut VTag>;
}

impl VTagExt for VTag {
    fn is_contains_class(&self, class: &str) -> bool {
        if let Some(classes) = self.attributes.get("class") {
            classes.split_whitespace().find(|&item| item == class).is_some()
        } else {
            false
        }
    }

    fn is_contains_any_class(&self, classes: &[&str]) -> bool {
        if let Some(class) = self.attributes.get("class") {
            class.split_whitespace().find(|item| classes.contains(item)).is_some()
        } else {
            false
        }
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

    fn is_first_child_contains_class(&self, class: &str) -> bool {
        if let Some(Html::VTag(child)) = self.children.first() {
            child.is_contains_class(class)
        } else {
            false
        }
    }

    fn is_some_child_contains_class(&self, class: &str) -> bool {
        for child in self.children.iter() {
            if child.is_contains_class(class) {
                return true;
            }
        }
        false
    }

    fn find_child_contains_class(&self, class: &str) -> Option<usize> {
        self.children.iter().enumerate().find_map(|(idx, child)| match child {
            Html::VTag(child) if child.is_contains_class(class) => Some(idx),
            _ => None,
        })
    }

    fn is_first_child(&self, child_tag_name: &str) -> bool {
        if let Some(Html::VTag(child)) = self.children.first() {
            child.tag() == child_tag_name
        } else {
            false
        }
    }

    fn is_last_child(&self, child_tag_name: &str) -> bool {
        if let Some(Html::VTag(child)) = self.children.last() {
            child.tag() == child_tag_name
        } else {
            false
        }
    }

    fn first_child_tag(&self, child_tag_name: &str) -> Option<&VTag> {
        match self.children.first() {
            Some(Html::VTag(child)) if child.tag() == child_tag_name => Some(child),
            _ => None,
        }
    }

    fn first_child_tag_mut(&mut self, child_tag_name: &str) -> Option<&mut VTag> {
        match self.children.first_mut() {
            Some(Html::VTag(child)) if child.tag() == child_tag_name => Some(child),
            _ => None,
        }
    }

    fn find_child_tag(&self, child_tag_name: &str) -> Option<&VTag> {
        for child in self.children.iter() {
            if let Html::VTag(child) = child {
                if child.tag() == child_tag_name {
                    return Some(child);
                }
            }
        }
        None
    }

    fn find_child_tag_mut(&mut self, child_tag_name: &str) -> Option<&mut VTag> {
        for child in self.children.iter_mut() {
            if let Html::VTag(child) = child {
                if child.tag() == child_tag_name {
                    return Some(child);
                }
            }
        }
        None
    }
}

impl VTagExt for Html {
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

    fn is_first_child_contains_class(&self, class: &str) -> bool {
        if let Html::VTag(tag) = self {
            tag.is_first_child_contains_class(class)
        } else {
            false
        }
    }

    fn is_some_child_contains_class(&self, class: &str) -> bool {
        if let Html::VTag(tag) = self {
            tag.is_some_child_contains_class(class)
        } else {
            false
        }
    }

    fn find_child_contains_class(&self, class: &str) -> Option<usize> {
        if let Html::VTag(tag) = self {
            tag.find_child_contains_class(class)
        } else {
            None
        }
    }

    fn is_first_child(&self, child_tag_name: &str) -> bool {
        if let Html::VTag(tag) = self {
            tag.is_first_child(child_tag_name)
        } else {
            false
        }
    }

    fn is_last_child(&self, child_tag_name: &str) -> bool {
        if let Html::VTag(tag) = self {
            tag.is_last_child(child_tag_name)
        } else {
            false
        }
    }

    fn first_child_tag(&self, child_tag_name: &str) -> Option<&VTag> {
        if let Html::VTag(tag) = self {
            tag.first_child_tag(child_tag_name)
        } else {
            None
        }
    }

    fn first_child_tag_mut(&mut self, child_tag_name: &str) -> Option<&mut VTag> {
        if let Html::VTag(tag) = self {
            tag.first_child_tag_mut(child_tag_name)
        } else {
            None
        }
    }

    fn find_child_tag(&self, child_tag_name: &str) -> Option<&VTag> {
        if let Html::VTag(tag) = self {
            tag.find_child_tag(child_tag_name)
        } else {
            None
        }
    }

    fn find_child_tag_mut(&mut self, child_tag_name: &str) -> Option<&mut VTag> {
        if let Html::VTag(tag) = self {
            tag.find_child_tag_mut(child_tag_name)
        } else {
            None
        }
    }
}

pub trait MdcWidget {
    const NAME: &'static str;

    fn html(&self) -> &Html;

    fn html_mut(&mut self) -> &mut Html;

    fn root_tag(&self) -> &VTag {
        match self.html() {
            Html::VTag(tag) => return tag,
            Html::VList(list) => if let Some(Html::VTag(tag)) = list.children.first() {
                return tag;
            },
            _ => (),
        }
        panic!("The root element of the {} must be a tag!", Self::NAME);
    }

    fn root_tag_mut(&mut self) -> &mut VTag {
        match self.html_mut() {
            Html::VTag(tag) => return tag,
            Html::VList(list) => if let Some(Html::VTag(tag)) = list.children.first_mut() {
                return tag;
            },
            _ => (),
        }
        panic!("The root element of the {} must be a tag!", Self::NAME);
    }
}

pub fn ripple(widget: &mut impl MdcWidget, ripple_class: impl AsRef<str>, enabled: bool) {
    let ripple_class = ripple_class.as_ref();
    if enabled {
        if !widget.root_tag().is_some_child_contains_class(ripple_class) {
            let idx = widget.root_tag().children.len().checked_sub(1).unwrap_or(0);
            widget.root_tag_mut().children.insert(idx, html! {
                <div class = ripple_class></div>
            });
        }
    } else {
        if let Some(idx) = widget.root_tag().find_child_contains_class(ripple_class) {
            widget.root_tag_mut().children.remove(idx);
        }
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