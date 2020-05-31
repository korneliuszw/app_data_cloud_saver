use gio::prelude::*;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder};
use std::cell::RefCell;

thread_local! {
    static GLOBAL: RefCell<(Option<gtk::TreeView>, Option<crate::apps::Apps>)> = RefCell::new((None, None))
}
pub struct UIBuilder<'a> {
    pub app: &'a gtk::Application,
    pub builder: gtk::Builder,
}
impl<'a> UIBuilder<'a> {
    pub fn obtain_builder(app: &'a gtk::Application) -> UIBuilder {
        let builder = Builder::new_from_string(include_str!("ui.glade"));
        UIBuilder { app, builder }
    }
    pub fn create_window(&self, name: &str) -> gtk::Window {
        let window: gtk::Window = self
            .builder
            .get_object(name)
            .expect("Could not spawn main window!");
        window.set_application(Some(self.app));
        window.show_all();
        window
    }
    pub fn create_main_window(&mut self) {
        self.create_window("window1");
        self.render_apps();
    }
    fn render_apps(&mut self) {
        let model = gtk::ListStore::new(&[u32::static_type(), String::static_type()]);
        let apps = crate::apps::Apps::read().unwrap();
        apps.apps.iter().enumerate().for_each(|(index, value)| {
            model.insert_with_values(None, &[0, 1], &[&(index as u32 + 1), &value.name]);
        });
        let tree: gtk::TreeView = self.builder.get_object("app_view").unwrap();
        tree.set_model(Some(&model));
        let name_entry: gtk::Entry = self.builder.get_object("name_entry").unwrap();
        let process_selector: gtk::FileChooserButton =
            self.builder.get_object("process_selector").unwrap();
        process_selector.add_filter(&construct_filter("Executable files", "*.exe"));
        let save_selector: gtk::FileChooserButton =
            self.builder.get_object("save_selector").unwrap();
        save_selector.add_filter(&construct_filter("All files", "*.*"));
        tree.connect_cursor_changed(move |tree_view| {
            let selection = tree_view.get_selection();
            if let Some((model, iter)) = selection.get_selected() {
                GLOBAL.with(|global| {
                    if let (_, Some(ref apps)) = *global.borrow() {
                        let app_option = &apps
                            .apps
                            .get(model.get_value(&iter, 0).get_some::<u32>().unwrap() as usize);
                        if let Some(app) = app_option {
                            name_entry.set_text(&app.name.clone());
                            process_selector.select_uri(&app.executable.clone());
                            save_selector.select_uri(&app.upload_path.clone());
                        }
                    }
                })
            };
        });
        GLOBAL.with(|global| {
            *global.borrow_mut() = (Some(tree), Some(apps));
        });
    }
}
fn construct_filter(name: &str, pattern: &str) -> gtk::FileFilter {
    let filter = gtk::FileFilter::new();
    filter.add_pattern(pattern);
    filter.set_name(Some(name));
    filter
}
