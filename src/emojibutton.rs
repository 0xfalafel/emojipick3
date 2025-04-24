use gtk::prelude::*;
use relm4::prelude::*;
use relm4::factory::Position;
use relm4::factory::positions::GridPosition;
use serde::Deserialize;

use crate::Msg;

#[derive(Debug, Deserialize, Clone)]
pub struct EmojiButton {
    pub symbol: String,
    pub name: String,
}

#[relm4::factory(pub)]
impl FactoryComponent for EmojiButton {
    type Init = (String, String);
    type Input = Msg;
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::Grid;

    view! {
        gtk::Button {
            set_label: &self.symbol,
            set_tooltip: &self.name,
            connect_clicked[symbol = self.symbol.clone()] => move |_| {
                println!("You clicked {}", symbol);
            },

            connect_clicked[sender, emoji = self.clone()] => move |_| {
                sender.input(Msg::Clicked(emoji.symbol.to_owned(), emoji.name.to_owned()))
            },
            // connect_clicked[symbol = self.symbol.clone()] => move |_| {
            //     println!("You clicked {}", symbol);
            // }
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        let (symbol, name) = init;
        Self { symbol, name }
    }
}

impl Position<GridPosition, DynamicIndex> for EmojiButton {
    fn position(&self, index: &DynamicIndex) -> GridPosition {
        let index = index.current_index();
        let x = index / 8;
        let y = index % 8;
        GridPosition {
            column: y as i32,
            row: x as i32,
            width: 1,
            height: 1,
        }
    }
}
