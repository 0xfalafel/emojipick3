use gtk::prelude::*;
use relm4::prelude::*;


use crate::emojibutton::EmojiButton;
// use crate::emojibutton::{EmojiButton, EmojiMsg};

// const SMILE_FACES: &str = include_str!("../data/smile_and_faces.json");
// const FOOD_DRINK: &str = include_str!("../data/food_and_drink.json");
// const ANIMALS_NATURE: &str = include_str!("../data/animals_and_nature.json");

#[derive(Debug)]
pub struct SearchResults {
    _emoji: FactoryVecDeque<EmojiButton>,
}

#[relm4::component(pub)]
impl SimpleComponent for SearchResults {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {

        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            gtk::Label {
                set_label: "Search Results"
            },

            #[name = "emoji_res"]
            gtk::Grid {

            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let emoji_res = gtk::Grid::new();

        let mut emoji: FactoryVecDeque<EmojiButton> = FactoryVecDeque::builder()
            .launch(emoji_res.clone())
            .detach();
            // .forward(sender.input_sender(), |msg| match msg {
            //     EmojiMsg::Clicked(symbol, name) => Msg::Clicked(symbol, name),
            // });

        // Use the Factory to create all the emoji buttons
        {
            let mut guard = emoji.guard();

            guard.push_back(("🐨".to_string(), "koala".to_string()));
        }


        let model = SearchResults {
            _emoji: emoji,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}