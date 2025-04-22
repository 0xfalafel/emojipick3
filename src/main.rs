use gtk::prelude::*;
use relm4::prelude::*;
use relm4::factory::FactoryVecDeque;

mod emojibutton;
use emojibutton::EmojiButton;

struct App {
    _emojis: FactoryVecDeque<EmojiButton>,
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

                gtk::Label {
                    set_label: "Smile and Faces",
                },

                #[local]
                smile_grid -> gtk::Grid {
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
        let smile_grid = gtk::Grid::default();

        let mut emojis_smile = FactoryVecDeque::builder()
            .launch(smile_grid.clone())
            .forward(sender.input_sender(), |output| output);

        const SMILE_FACES: &str = include_str!("../data/smile_and_faces.json");

        let emojis_smile_list: Vec<EmojiButton> = serde_json::from_str(SMILE_FACES).unwrap();

        // Initialize a counter
        {
            let mut guard = emojis_smile.guard();

            for emoji in emojis_smile_list {
                guard.push_back((emoji.symbol, emoji.name));
            }
        }

            
        let model = App {
            _emojis: emojis_smile,
            entry: gtk::EntryBuffer::default(),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

fn main() {
    let app = RelmApp::new("relm4.example.entry");
    app.run::<App>(());
}
