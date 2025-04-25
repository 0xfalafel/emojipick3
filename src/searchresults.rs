use relm4::gtk;
use relm4::prelude::*;
use relm4::component;

#[derive(Default)]
pub struct SearchResults {}

#[derive(Debug)]
pub enum SearchResultsMsg {}

#[component(pub)]
impl SimpleComponent for SearchResults {
    type Init = ();
    type Input = SearchResultsMsg;
    type Output = ();

    view! {
        #[root]
        gtk::Box {
            gtk::Label {
                set_label: "Search results"
            }
        }
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = SearchResults::default();
        let widgets = view_output!();

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
