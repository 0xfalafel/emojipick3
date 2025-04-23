use gtk::prelude::*;
use relm4::gtk::{gdk, Grid};
use relm4::prelude::*;
use relm4::factory::FactoryVecDeque;

mod emojibutton;
use emojibutton::EmojiButton;

const SMILE_FACES: &str = include_str!("../data/smile_and_faces.json");

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
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        load_css();

        let smile_grid = gtk::Grid::default();
        let emojis_smile= initialize_emoji_grid(SMILE_FACES, &smile_grid);

        
            
        let model = App {
            _emojis: emojis_smile,
            entry: gtk::EntryBuffer::default(),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

/// Initialize a grid of emojis from a json containing the emojis
fn initialize_emoji_grid(json_emojis: &str, grid: &Grid) -> FactoryVecDeque<EmojiButton> {
    let mut emoji_buttons: FactoryVecDeque<EmojiButton> = FactoryVecDeque::builder()
        .launch(grid.clone())
        .detach();

    let emojis_list: Vec<EmojiButton> = serde_json::from_str(json_emojis).unwrap();

    // Use the Factory to create all the emoji buttons
    {
        let mut guard = emoji_buttons.guard();

        for emoji in emojis_list {
            guard.push_back((emoji.symbol, emoji.name));
        }
    }

    emoji_buttons
}

/// from https://jamesbenner.hashnode.dev/how-to-style-your-gtk4-rust-app-with-css
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
