use relm4::gtk;
use relm4::gtk::prelude::{GridExt, OrientableExt, WidgetExt};
use relm4::prelude::*;

use crate::emojibutton::EmojiButton;

#[derive(Default, Debug)]
pub struct SearchResults {}

// #[derive(Debug)]
// pub enum SearchResultsMsg {}

#[relm4::component(pub)]
impl SimpleComponent for SearchResults {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        gtk::ScrolledWindow {
            gtk::Label {
                set_label: "Search results"
            },

            #[name = "emoji_grid"]
            gtk::Grid {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 5,
                set_column_spacing: 15,
                set_row_spacing: 15,
                add_css_class: "emojigrid",
            }
        }
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = SearchResults {};
        let widgets = view_output!();

        let emoji_grid = gtk::Grid::default();

        let mut res_emojis: FactoryVecDeque<EmojiButton> = FactoryVecDeque::builder()
            .launch(emoji_grid.clone())
            .detach();
            // .forward(sender.input_sender(), |msg| match msg {
            //     Msg::Clicked(symbol, name) => Msg::Clicked(symbol, name),
            //     _ => todo!()
            // });

        ComponentParts { model, widgets }
    }

    // fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
    //     match msg {
    //         SearchResultsMsg::SwitchPage(page) => {
    //             println!("clicked page {}", page);
    //             self.current_page = page;
    //             self.stack.set_visible_child_name("page1");
    //         }
    //     }
    // }

}
