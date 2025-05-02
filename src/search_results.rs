use gtk::prelude::*;
use relm4::prelude::*;
use serde::Deserialize;


use crate::emojibutton::{EmojiButton, EmojiMsg};
// use crate::emojibutton::{EmojiButton, EmojiMsg};

const SMILE_FACES: &str = include_str!("../data/smile_and_faces.json");
const FOOD_DRINK: &str = include_str!("../data/food_and_drink.json");
const ANIMALS_NATURE: &str = include_str!("../data/animals_and_nature.json");

#[derive(Debug, Deserialize, Clone)]
struct Emoji {
    symbol: String,
    name: String,
}

#[derive(Debug)]
pub enum SearchMsg {
    SearchedText(String),
    Clicked(String, String),
}

#[derive(Debug)]
pub struct SearchResults {
    emoji: FactoryVecDeque<EmojiButton>,
    all_emojis: Vec<Emoji>,
}

#[relm4::component(pub)]
impl Component for SearchResults {
    type Init = ();
    type Input = SearchMsg;
    type Output = SearchMsg;
    type CommandOutput = ();

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_margin_all: 5,
            set_spacing: 5,
            set_vexpand: true,
            set_hexpand: true,

            gtk::ScrolledWindow {
                set_vexpand: true,

                gtk::Label {
                    set_label: "Search Results"
                },

                #[name = "emoji_res"]
                gtk::Grid {
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_all: 5,
                    set_column_spacing: 15,
                    set_row_spacing: 15,
                },
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

        let emoji: FactoryVecDeque<EmojiButton> = FactoryVecDeque::builder()
            .launch(emoji_res)
            .forward(sender.input_sender(), |msg| match msg {
                EmojiMsg::Clicked(symbol, name) => SearchMsg::Clicked(symbol, name),
            });

        // Parse JSON data
        let mut all_emojis: Vec<Emoji> = serde_json::from_str(SMILE_FACES).unwrap();
        all_emojis.extend(
            serde_json::from_str::<Vec<Emoji>>(FOOD_DRINK).unwrap()
        );
        all_emojis.extend(
            serde_json::from_str::<Vec<Emoji>>(ANIMALS_NATURE).unwrap()
        );

        let model: SearchResults = SearchResults {
            emoji: emoji,
            all_emojis: all_emojis,
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            SearchMsg::Clicked(symbol, name) => {
                let _ = sender.output(SearchMsg::Clicked(symbol, name));
            },
            SearchMsg::SearchedText(search) => {
                // Search all emojis match the text searched
                let search_res: Vec<(String, String)> = self.all_emojis
                    .iter()
                    .filter(|emoji| emoji.name.contains(&search))
                    .map(|emoji| (emoji.symbol.clone(), emoji.name.clone()))
                    .collect();
                
                // Update the emoji_res Grid with filtered emojis
                let mut guard = self.emoji.guard();
                guard.clear();
                for (symbol, name) in search_res {
                    guard.push_back((symbol, name));
                }
                
            }
        }
    }

}