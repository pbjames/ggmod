use crate::modz::LocalCollection;

pub struct App<'a> {
    pub view: View,
    mods: &'a mut LocalCollection,
}

pub enum View {
    Manage,
    Browse,
}

impl<'a> App<'a> {
    pub fn new(collection: &mut LocalCollection) -> App {
        App {
            view: View::Manage,
            mods: collection,
        }
    }

    pub fn toggle_view(&mut self) {
        match self.view {
            View::Manage => self.view = View::Browse,
            View::Browse => self.view = View::Manage,
        }
    }
}
