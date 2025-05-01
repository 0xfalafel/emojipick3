use relm4::prelude::*;

// use crate::emojibutton::{EmojiButton, EmojiMsg};

// const SMILE_FACES: &str = include_str!("../data/smile_and_faces.json");
// const FOOD_DRINK: &str = include_str!("../data/food_and_drink.json");
// const ANIMALS_NATURE: &str = include_str!("../data/animals_and_nature.json");

#[derive(Debug, Default)]
pub struct SearchResults;

#[relm4::component(pub)]
impl SimpleComponent for SearchResults {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {

        gtk::Box {

            gtk::Label {
                set_label: "Search Results"
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        
        let model = SearchResults;

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}