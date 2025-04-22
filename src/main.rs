use gtk::prelude::*;
use relm4::factory::{FactoryVecDeque, Position};
use relm4::factory::positions::GridPosition;
use relm4::prelude::*;

struct Emoji {
    pub symbol: String,
    pub name: String,
}

struct App {
    emojis: FactoryVecDeque<Emoji>,
    entry: gtk::EntryBuffer,
}

#[relm4::factory]
impl FactoryComponent for Emoji {
    type Init = (String, String);
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::Grid;

    view! {
        gtk::Button {
            #[watch]
            set_label: &self.symbol,
            set_tooltip: &self.name,
            connect_clicked[index] => move |_| {
                println!("You clicked me !");
            },
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        let (symbol, name) = init;
        Self { symbol, name }
    }
}

impl Position<GridPosition, DynamicIndex> for Emoji {
    fn position(&self, index: &DynamicIndex) -> GridPosition {
        let index = index.current_index();
        let x = index / 10;
        let y = index % 10;
        GridPosition {
            column: y as i32,
            row: x as i32,
            width: 1,
            height: 1,
        }
    }
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        gtk::ApplicationWindow {
            set_title: Some("Entry example"),
            set_default_size: (300, 200),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 5,
                set_spacing: 5,

                gtk::Entry {
                    set_buffer: &model.entry,
                    set_tooltip_text: Some("Search for emojis"),
                },

                #[local]
                factory_box -> gtk::Grid {
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
        let factory_box = gtk::Grid::default();

        let emojis = FactoryVecDeque::builder()
            .launch(factory_box.clone())
            .forward(sender.input_sender(), |output| output);

        let mut model = App {
            emojis,
            entry: gtk::EntryBuffer::default(),
        };

        // Initialize a counter
        {
            let mut guard = model.emojis.guard();
            guard.push_back(("😄".to_string(), "smile".to_string()));
        }

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

fn main() {
    let app = RelmApp::new("relm4.example.entry");
    app.run::<App>(());
}
