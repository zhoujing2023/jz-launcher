use crate::app_data_object::AppDataObject;
use glib::object::ObjectExt;
use glib::subclass::prelude::{ObjectImpl, ObjectSubclass};
use gtk::subclass::prelude::{BoxImpl, CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl};
use gtk::subclass::widget::WidgetClassExt;
use gtk::{CompositeTemplate, TemplateChild};

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/zhoujing/jz_tools/ui/search-result-item.ui")]
pub struct SearchResultItem {
    #[template_child]
    pub icon: TemplateChild<gtk::Image>,
    #[template_child]
    pub name_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub comment_label: TemplateChild<gtk::Label>,
}

#[glib::object_subclass]
impl ObjectSubclass for SearchResultItem {
    const NAME: &'static str = "SearchResultItem";
    type Type = super::SearchResultItem;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for SearchResultItem {}

impl WidgetImpl for SearchResultItem {}

impl BoxImpl for SearchResultItem {}

impl SearchResultItem {
    /// `bind` 绑定属性
    pub(super) fn bind(&self, app_info: Option<AppDataObject>) {
        if let Some(info) = app_info {
            info.bind_property("name", &self.name_label.get(), "label")
                .sync_create()
                .build();
            info.bind_property("comment", &self.comment_label.get(), "label")
                .sync_create()
                .build();
            info.bind_property("icon_path", &self.icon.get(), "file")
                .sync_create()
                .build();
        }
    }
}
