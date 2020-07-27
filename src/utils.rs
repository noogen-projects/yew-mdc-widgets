use yew::{Html, virtual_dom::VTag};

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

    fn find_child_tag_mut(&mut self, child_tag_name: &str) -> Option<&mut VTag> {
        if let Html::VTag(tag) = self {
            tag.find_child_tag_mut(child_tag_name)
        } else {
            None
        }
    }
}