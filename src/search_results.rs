use relm4::prelude::*;

// use crate::emojibutton::{EmojiButton, EmojiMsg};

const SMILE_FACES: &str = include_str!("../data/smile_and_faces.json");
const FOOD_DRINK: &str = include_str!("../data/food_and_drink.json");
const ANIMALS_NATURE: &str = include_str!("../data/animals_and_nature.json");

#[derive(Debug)]
pub enum Msg {
    SearchedText(String),
    Clicked(String, String),
}

pub struct SearchResults;
// {
//     _emojis: FactoryVecDeque<EmojiButton>,
// }

#[relm4::component]
impl Component for SearchResults {
    type Init = ();
    type Input = Msg;
    type Output = ();
    type CommandOutput = ();
    type Widgets = SearchResultsWidgets;

    view! {
        gtk::Label {
            set_label: "Search Results"
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        
        let model = SearchResults;
        //  {
        //     _emojis_smiles: emojis_smile,
        //     _emojis_food: emojis_food,
        //     _emojis_animals: emojis_animals,
        //     entry: gtk::EntryBuffer::default(),
        //     stack: gtk::Stack::default(),
        // };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

// /// Initialize a grid of emojis from a json containing the emojis
// fn initialize_emoji_grid(json_emojis: &str, grid: &Grid, sender: ComponentSender<SearchResults>) -> FactoryVecDeque<EmojiButton> {
//     let mut emoji_buttons: FactoryVecDeque<EmojiButton> = FactoryVecDeque::builder()
//         .launch(grid.clone())
//         .forward(sender.input_sender(), |msg| match msg {
//             EmojiMsg::Clicked(symbol, name) => Msg::Clicked(symbol, name),
//         });


//     let emojis_list: Vec<EmojiButton> = serde_json::from_str(json_emojis).unwrap();

//     // Use the Factory to create all the emoji buttons
//     {
//         let mut guard = emoji_buttons.guard();

//         for emoji in emojis_list {
//             guard.push_back((emoji.symbol, emoji.name));
//         }
//     }

//     emoji_buttons
// }
