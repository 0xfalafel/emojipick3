use gtk::prelude::*;
use relm4::gtk::{gdk, Grid};
use relm4::prelude::*;
use relm4::factory::FactoryVecDeque;

mod emojibutton;
use emojibutton::EmojiButton;

const SMILE_FACES: &str = include_str!("../data/smile_and_faces.json");
const FOOD_DRINK: &str = include_str!("../data/food_and_drink.json");
const ANIMALS_NATURE: &str = include_str!("../data/animals_and_nature.json");

struct App {
    _emojis_smiles: FactoryVecDeque<EmojiButton>,
    _emojis_food: FactoryVecDeque<EmojiButton>,
    _emojis_animals: FactoryVecDeque<EmojiButton>,
    entry: gtk::EntryBuffer,
    stack: gtk::Stack,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        gtk::ApplicationWindow {
            // set_title: Some("Entry example"),
            set_default_size: (477, 400),
            set_resizable: false,

            #[wrap(Some)]
            set_titlebar = &gtk::Grid::new(),

            gtk::WindowHandle {

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_all: 5,
                    set_spacing: 5,
                    
                    gtk::HeaderBar {
                        set_show_title_buttons: false,
                        pack_start = &gtk::WindowControls::new(gtk::PackType::Start),
                        pack_end = &gtk::WindowControls::new(gtk::PackType::End),
                        add_css_class: "flat",
                        
                        #[wrap(Some)]
                        set_title_widget = &gtk::Entry {
                            set_buffer: &model.entry,
                            set_tooltip_text: Some("Search for emojis"),
                            
                            // focus the searchbar when launching the app
                            // connect_realize => move |entry| {
                            //     entry.grab_focus();
                            // },
                            
                            connect_changed => move |entry| {
                                let search = entry.text().to_string();
                                println!("{}", search);
                            },
                        },
                    },
                    
                    
                    gtk::ScrolledWindow {
                        set_vexpand: true,
                        
                        #[name = "stack"]
                        gtk::Stack {

                            add_child = &gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                
                                // Smile and Faces
                                gtk::Label::new(Some("Smile and Faces")),
                                
                                #[local]
                                smile_grid -> gtk::Grid {
                                    set_orientation: gtk::Orientation::Vertical,
                                    set_margin_all: 5,
                                    set_column_spacing: 15,
                                    set_row_spacing: 15,
                                },
                                
                                // Food and Drinks
                                gtk::Label::new(Some("Food and Drinks")),
                                
                                #[local]
                                food_grid -> gtk::Grid {
                                    set_orientation: gtk::Orientation::Vertical,
                                    set_margin_all: 5,
                                    set_column_spacing: 15,
                                    set_row_spacing: 15,
                                },
                                
                                // Animals and Nature
                                gtk::Label::new(Some("Animals and Nature")),
                                
                                #[local]
                                animals_grid -> gtk::Grid {
                                    set_orientation: gtk::Orientation::Vertical,
                                    set_margin_all: 5,
                                    set_column_spacing: 15,
                                    set_row_spacing: 15,
                                },
                            } -> {
                                set_name: "emoji_list",
                            },

                            add_child = &gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,

                                gtk::Label {
                                    set_label: "Search Results"
                                }
                            } -> {
                                set_name: "search_results",
                            }
                        }
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

        let food_grid = gtk::Grid::default();
        let emojis_food= initialize_emoji_grid(FOOD_DRINK, &food_grid);

        let animals_grid = gtk::Grid::default();
        let emojis_animals= initialize_emoji_grid(ANIMALS_NATURE, &animals_grid);

        
        let mut model = App {
            _emojis_smiles: emojis_smile,
            _emojis_food: emojis_food,
            _emojis_animals: emojis_animals,
            entry: gtk::EntryBuffer::default(),
            stack: gtk::Stack::default(),
        };

        let widgets = view_output!();

        model.stack = widgets.stack.clone();

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
