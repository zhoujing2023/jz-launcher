use crate::app_data_object::AppDataObject;
use glib::Properties;
use glib::object::ObjectExt;
use glib::subclass::object::{ObjectImpl, ObjectImplExt};
use glib::subclass::prelude::ObjectSubclass;
use glib::subclass::types::ObjectSubclassExt;
use gtk::pango::EllipsizeMode;
use gtk::prelude::{BoxExt, OrientableExt, WidgetExt};
use gtk::subclass::prelude::{BoxImpl, WidgetImpl};
use gtk::{Image, Label};
use std::cell::RefCell;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::SearchResultItem)]
pub struct SearchResultItem {
    pub icon: RefCell<gtk::Image>,
    pub name_label: RefCell<Label>,
    pub comment_label: RefCell<Label>,
}

#[glib::object_subclass]
impl ObjectSubclass for SearchResultItem {
    const NAME: &'static str = "MySearchResultItem";
    type Type = super::SearchResultItem;
    type ParentType = gtk::Box;
}

impl ObjectImpl for SearchResultItem {
    fn constructed(&self) {
        self.parent_constructed();
        // 获取类型实例（gtk::Box）
        let this = self.obj();
        this.set_orientation(gtk::Orientation::Horizontal);
        this.set_spacing(10);
        this.set_margin_top(10);
        this.set_margin_bottom(10);
        this.set_margin_start(10);
        this.set_margin_end(10);

        // 创建子控件
        let icon = Image::builder().pixel_size(48).build();
        let text_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(4)
            .build();
        let name_label = Label::builder()
            .halign(gtk::Align::Start)
            .build();
        let comment_label = Label::builder().halign(gtk::Align::Start)
            .max_width_chars(40)
            .ellipsize(EllipsizeMode::End)
            .build();

        text_box.append(&name_label);
        text_box.append(&comment_label);
        this.append(&icon);
        this.append(&text_box);

        // 保存子控件，用于后续“绑定”
        *self.name_label.borrow_mut() = name_label;
        *self.icon.borrow_mut() = icon;
        *self.comment_label.borrow_mut() = comment_label;
    }
}

impl WidgetImpl for SearchResultItem {}

impl BoxImpl for SearchResultItem {}

impl SearchResultItem {
    pub(crate) fn bind(&self, app_data_obj: Option<AppDataObject>) {
        if let Some(app_data) = app_data_obj {
            app_data
                .bind_property("name", &*self.name_label.borrow(), "label")
                .sync_create()
                .build();
            app_data
                .bind_property("icon", &*self.icon.borrow(), "file")
                .sync_create()
                .build();
            app_data
                .bind_property("comment", &*self.comment_label.borrow(), "label")
                .sync_create()
                .build();
        }
    }
}
