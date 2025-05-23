use gtk::prelude::*;
use relm4::prelude::*;
use relm4::factory::Position;
use relm4::factory::positions::GridPosition;
use serde::Deserialize;

#[derive(Debug)]
pub enum EmojiMsg {
    Clicked(String, String),
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmojiButton {
    pub symbol: String,
    pub name: String,
}

#[relm4::factory(pub)]
impl FactoryComponent for EmojiButton {
    type Init = (String, String);
    type Input = ();
    type Output = EmojiMsg;
    type CommandOutput = ();
    type ParentWidget = gtk::Grid;

    view! {
        gtk::Button {
            set_label: &self.symbol,
            set_tooltip: &self.name,

            connect_clicked[sender, emoji = self.clone()] => move |_| {
                if let Err(e) = sender.output(EmojiMsg::Clicked(emoji.symbol.clone(), emoji.name.clone())) {
                    eprintln!("Failed to send message from EmojiButton: {:?}", e);
                }
            },
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
