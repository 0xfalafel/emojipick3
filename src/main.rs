use gtk::prelude::*;
use relm4::prelude::*;
use relm4::factory::FactoryVecDeque;

mod emojibutton;
use emojibutton::EmojiButton;

struct App {
    emojis: FactoryVecDeque<EmojiButton>,
    entry: gtk::EntryBuffer,
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
                simle_emojis -> gtk::Grid {
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
        let simle_emojis = gtk::Grid::default();

        let emojis = FactoryVecDeque::builder()
            .launch(simle_emojis.clone())
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
