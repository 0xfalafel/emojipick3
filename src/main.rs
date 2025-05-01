use gtk::prelude::*;
use relm4::gtk::gdk::{Display, Key};
use relm4::gtk::{gdk, Grid};
use relm4::prelude::*;
use relm4::factory::FactoryVecDeque;
use std::process::Command;
use std::time::Duration;

mod emojibutton;
use emojibutton::{EmojiButton, EmojiMsg};

mod search_results;
use search_results::{SearchResults, SearchMsg};

const SMILE_FACES: &str = include_str!("../data/smile_and_faces.json");
const FOOD_DRINK: &str = include_str!("../data/food_and_drink.json");
const ANIMALS_NATURE: &str = include_str!("../data/animals_and_nature.json");

#[derive(Debug)]
pub enum Msg {
    SearchedText(String),
    Clicked(String, String),
    Quit,
}

struct App {
    _emojis_smiles: FactoryVecDeque<EmojiButton>,
    _emojis_food: FactoryVecDeque<EmojiButton>,
    _emojis_animals: FactoryVecDeque<EmojiButton>,
    entry: gtk::EntryBuffer,
    stack: gtk::Stack,
    search_res: Controller<SearchResults>,
}

#[relm4::component]
impl Component for App {
    type Init = ();
    type Input = Msg;
    type Output = ();
    type CommandOutput = ();
    type Widgets = AppWidgets;

    view! {
        #[root]
        gtk::ApplicationWindow {
            set_default_size: (493, 400),
            set_resizable: false,

            #[wrap(Some)]
            set_titlebar = &gtk::Grid::new(),

            /// Quit the App when `Esc` is pressed
            add_controller = gtk::EventControllerKey {
                connect_key_pressed[sender] => move |_, key, _, _| {
                    if key == Key::Escape {
                        sender.input(Msg::Quit);
                    }
                    gtk::glib::Propagation::Proceed
                }
            },
            
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
                            set_placeholder_text: Some("Search for emojis"),
                            set_tooltip_text: Some("Search for emojis"),

                            // focus the searchbar when launching the app
                            // connect_realize => move |entry| {
                            //     entry.grab_focus();
                            // },
                            
                            connect_changed[sender] => move |entry| {
                                let text = entry.text().to_string();
                                sender.input(Msg::SearchedText(text))
                            }
                        },
                    },
                    
                    
                    #[name = "stack"]
                    gtk::Stack {

                        add_child = &gtk::ScrolledWindow {
                            set_vexpand: true,
                            
                            gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                // Smile and Faces
                                gtk::Label {
                                    set_label: "Smile and Faces",
                                    add_css_class: "category",
                                },
                                
                                #[local]
                                smile_grid -> gtk::Grid {
                                    set_orientation: gtk::Orientation::Vertical,
                                    set_margin_all: 5,
                                    set_column_spacing: 15,
                                    set_row_spacing: 15,
                                    add_css_class: "emojigrid",
                                },
                                
                                // Food and Drinks
                                gtk::Label {
                                    set_label: "Food and Drinks",
                                    add_css_class: "category",
                                },
                                
                                #[local]
                                food_grid -> gtk::Grid {
                                    set_orientation: gtk::Orientation::Vertical,
                                    set_margin_all: 5,
                                    set_column_spacing: 15,
                                    set_row_spacing: 15,
                                    add_css_class: "emojigrid",
                                },
                                
                                // Animals and Nature
                                gtk::Label {
                                    set_label: "Animals and Nature",
                                    add_css_class: "category",
                                },
                                
                                #[local]
                                animals_grid -> gtk::Grid {
                                    set_orientation: gtk::Orientation::Vertical,
                                    set_margin_all: 5,
                                    set_column_spacing: 15,
                                    set_row_spacing: 15,
                                },
                            }
                        } -> {
                            set_name: "emoji_list",
                        },

                        add_child = &gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,

                            append: model.search_res.widget()

                        } -> {
                            set_name: "search_results",
                        }
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
        let emojis_smile= initialize_emoji_grid(SMILE_FACES, &smile_grid, sender.clone());

        let food_grid = gtk::Grid::default();
        let emojis_food= initialize_emoji_grid(FOOD_DRINK, &food_grid, sender.clone());

        let animals_grid = gtk::Grid::default();
        let emojis_animals= initialize_emoji_grid(ANIMALS_NATURE, &animals_grid, sender.clone());

        let search_res = SearchResults::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| match msg {
                SearchMsg::Clicked(symbol, name) => Msg::Clicked(symbol, name),
                SearchMsg::SearchedText(_) => unreachable!(),
            });
            
        let mut model = App {
            _emojis_smiles: emojis_smile,
            _emojis_food: emojis_food,
            _emojis_animals: emojis_animals,
            entry: gtk::EntryBuffer::default(),
            stack: gtk::Stack::default(),
            search_res: search_res,
        };

        let widgets = view_output!();

        model.stack = widgets.stack.clone();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>, root: &Self::Root) {
        match message {
            Msg::SearchedText(search) => {
                match search.len() {
                    0 => self.stack.set_visible_child_name("emoji_list"),
                    _ => self.stack.set_visible_child_name("search_results"),
                }

                self.search_res.emit(SearchMsg::SearchedText(search))
            },
            Msg::Clicked(symbol, _name) => {
                root.hide();
                println!("You clicked {}", symbol);
                
                // Copy the emoji to the clipboard
                if let Some(display) = Display::default() {
                    let clipboard = display.clipboard();
                    clipboard.set_text(&symbol);
                }
                                
                // Use a system command to paste the emoji
                // xdotool key ctrl+v
                Command::new("xdotool")
                    .args(["key", "ctrl+shift+v"])
                    .status()
                    .expect("Failed to execute command");
            
                // For some reason, the application close before the emoji is paste
                // with thread::sleep.
                gtk::glib::timeout_add_once(Duration::from_millis(1), move || {
                    relm4::main_application().quit();
                });
            },
            Msg::Quit => {
                relm4::main_application().quit();
            }
        }
    }
}

/// Initialize a grid of emojis from a json containing the emojis
fn initialize_emoji_grid(json_emojis: &str, grid: &Grid, sender: ComponentSender<App>) -> FactoryVecDeque<EmojiButton> {
    let mut emoji_buttons: FactoryVecDeque<EmojiButton> = FactoryVecDeque::builder()
        .launch(grid.clone())
        .forward(sender.input_sender(), |msg| match msg {
            EmojiMsg::Clicked(symbol, name) => Msg::Clicked(symbol, name),
        });


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
