use gio::prelude::*;
use gtk::prelude::*;
use gtk::Builder;
use std::cell::RefCell;
thread_local! {
    static GLOBAL: RefCell<(Option<gtk::TreeView>, Option<Vec<crate::apps::App>>)> = RefCell::new((None, None));
    static TREE_STORE: RefCell<Option<gtk::ListStore>> = RefCell::new(None)
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
        self.listen_for_settings_save();
        self.connect_remove_button();
        self.connect_add_button();
    }
    fn render_apps(&mut self) {
        let model = gtk::ListStore::new(&[u32::static_type(), String::static_type()]);
        let apps = crate::apps::Apps::read().unwrap().0;
        apps.iter().enumerate().for_each(|(index, value)| {
            model.insert_with_values(None, &[0, 1], &[&(index as u32 + 1), &value.name]);
        });
        let tree: gtk::TreeView = self.builder.get_object("app_view").unwrap();
        tree.set_headers_visible(false);
        append_column(&tree, 0);
        append_column(&tree, 1);
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
                            .get(model.get_value(&iter, 0).get_some::<u32>().unwrap() as usize - 1);
                        if let Some(app) = app_option {
                            name_entry.set_text(&app.name.clone());
                            name_entry.set_editable(false);
                            process_selector
                                .select_uri(&format!("file:///{}", &app.executable.clone()));
                            save_selector
                                .select_uri(&format!("file:///{}", &app.upload_path.clone()));
                        }
                    }
                })
            };
        });
        TREE_STORE.with(|tree_store| {
            *tree_store.borrow_mut() = Some(model);
        });
        GLOBAL.with(|global| {
            *global.borrow_mut() = (Some(tree), Some(apps));
        });
    }
    fn connect_add_button(&self) {
        let add_button : gtk::Button = self.builder.get_object("add_app").unwrap();
        let name_entry: gtk::Entry = self.builder.get_object("name_entry").unwrap();
        let process_selector: gtk::FileChooserButton =
            self.builder.get_object("process_selector").unwrap();
        let save_selector: gtk::FileChooserButton =
            self.builder.get_object("save_selector").unwrap();
        add_button.connect_clicked(move |_| {
            GLOBAL.with(|global| {
                if let (Some(ref tree), _) = *global.borrow_mut() {
                    tree.get_selection().unselect_all();
                }
            });
            name_entry.set_text("");
            name_entry.set_editable(true);
            process_selector.set_uri("");
            save_selector.set_uri("");
        });
    }
    fn connect_remove_button(&self) {
        let remove_button : gtk::Button = self.builder.get_object("remove_app_button").unwrap();
        remove_button.connect_clicked(|_| {
            GLOBAL.with(|global| {
                if let (Some(ref tree), Some(ref mut apps)) = *global.borrow_mut() {
                    if let Some((model, iter)) = tree.get_selection().get_selected() {
                        let index = model.get_value(&iter, 0).get_some::<u32>().unwrap() as usize - 1;
                        TREE_STORE.with(move |tree_store| {
                            if let Some(store) = &*tree_store.borrow() {
                                apps.get(index).unwrap().delete();
                                store.remove(&iter);
                            } 
                        })
                    } 
                }
            });
        });
    }
    fn listen_for_settings_save(&mut self) {
        let name_entry: gtk::Entry = self.builder.get_object("name_entry").unwrap();
        let process_selector: gtk::FileChooserButton =
            self.builder.get_object("process_selector").unwrap();
        let save_selector: gtk::FileChooserButton =
            self.builder.get_object("save_selector").unwrap();
        let settings_save_button: gtk::Button = self.builder.get_object("save_settings").unwrap();
        settings_save_button.connect_clicked(move |_| {
            let executable_uri = process_selector.get_uri();
            let upload_path = save_selector.get_uri();
            if executable_uri.is_none() || upload_path.is_none() {
                return;
            }
            let mut new_app = crate::apps::App {
                name: name_entry.get_text().unwrap().to_string(),
                executable: executable_uri
                    .unwrap()
                    .as_str()
                    .to_string()
                    .replace("file:///", ""),
                upload_path: upload_path
                    .unwrap()
                    .as_str()
                    .to_string()
                    .replace("file:///", "")
            };
            GLOBAL.with(move |global| {
                if let (Some(ref tree), Some(ref mut apps)) = *global.borrow_mut() {
                    new_app.save().unwrap();
                    if let Some((model, iter)) = tree.get_selection().get_selected() {
                        if let Some(ref mut app) = apps.get_mut(
                            model.get_value(&iter, 0).get_some::<u32>().unwrap() as usize - 1,
                        ) {
                            **app = new_app;
                            return;
                        }
                    }
                    TREE_STORE.with(|tree_sort| {
                        if let Some(ref tree) = *tree_sort.borrow() {
                            let tree_iter = tree.insert(-1);
                            tree.set(
                                &tree_iter,
                                &[0, 1],
                                &[&(apps.len() as u32 + 1), &new_app.name],
                            );
                        }
                    });
                    apps.push(new_app);
                } else {
                    global.borrow_mut().1 = Some(vec![new_app]);
                }
            });
        });
    }
}
fn construct_filter(name: &str, pattern: &str) -> gtk::FileFilter {
    let filter = gtk::FileFilter::new();
    filter.add_pattern(pattern);
    filter.set_name(Some(name));
    filter
}
fn append_column(tree: &gtk::TreeView, id: i32) {
    let column = gtk::TreeViewColumn::new();
    if id == 0 {
        column.set_visible(false);
    }
    let cell = gtk::CellRendererText::new();
    column.pack_start(&cell, true);
    // Association of the view's column with the model's `id` column.
    column.add_attribute(&cell, "text", id);
    tree.append_column(&column);
}
