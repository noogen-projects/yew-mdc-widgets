pub mod button;
pub mod table;
pub mod text_field;

pub trait Widget {
    fn build(&self) -> yew::Html;
}
