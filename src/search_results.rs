use gtk::prelude::*;
use relm4::prelude::*;


use crate::emojibutton::{EmojiButton, EmojiMsg};
// use crate::emojibutton::{EmojiButton, EmojiMsg};

// const SMILE_FACES: &str = include_str!("../data/smile_and_faces.json");
// const FOOD_DRINK: &str = include_str!("../data/food_and_drink.json");
// const ANIMALS_NATURE: &str = include_str!("../data/animals_and_nature.json");

#[derive(Debug)]
pub enum SearchMsg {
    SearchedText(String),
    Clicked(String, String),
}

#[derive(Debug)]
pub struct SearchResults {
    _emoji: FactoryVecDeque<EmojiButton>,
}

#[relm4::component(pub)]
impl SimpleComponent for SearchResults {
    type Init = ();
    type Input = SearchMsg;
    type Output = SearchMsg;

    view! {

        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            gtk::Label {
                set_label: "Search Results"
            },

            #[name = "emoji_res"]
            gtk::Grid {

            },

            #[name = "searched"]
            gtk::Label {
                set_label: "Hi mom!",
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {

        let widgets: SearchResultsWidgets = view_output!();

        let emoji_res = widgets.emoji_res.clone();

        let mut emoji: FactoryVecDeque<EmojiButton> = FactoryVecDeque::builder()
            .launch(emoji_res)
            .forward(sender.input_sender(), |msg| match msg {
                EmojiMsg::Clicked(symbol, name) => SearchMsg::Clicked(symbol, name),
            });

        // Use the Factory to create all the emoji buttons
        {
            let mut guard = emoji.guard();
            guard.push_back(("🐨".to_string(), "koala".to_string()));
        }


        let model: SearchResults = SearchResults {
            _emoji: emoji,
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            SearchMsg::Clicked(symbol, name) => {
                let _ = sender.output(SearchMsg::Clicked(symbol, name));
            },
            SearchMsg::SearchedText(search) => {
                println!("You searched this text: {}", search);
            }
        }
    }
}