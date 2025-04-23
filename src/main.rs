use gtk::prelude::*;
use relm4::gtk::gdk;
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
            set_default_size: (477, 400),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 5,
                set_spacing: 5,

                gtk::Entry {
                    set_buffer: &model.entry,
                    set_tooltip_text: Some("Search for emojis"),
                },

                gtk::ScrolledWindow {
                    set_vexpand: true,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,

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
                },
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        load_css();


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

// from https://jamesbenner.hashnode.dev/how-to-style-your-gtk4-rust-app-with-css
fn load_css() {
    let display = gdk::Display::default().expect("Could not get default display.");
    let provider = gtk::CssProvider::new();
    let priority = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;

    // load our custom CSS
    provider.load_from_data(include_str!("../data/style.css"));
    gtk::style_context_add_provider_for_display(&display, &provider, priority);
}

fn main() {
    let app = RelmApp::new("io.github.falafel.emojipick");
    app.run::<App>(());
}
