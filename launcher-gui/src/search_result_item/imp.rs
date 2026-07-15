use crate::app_data_object::AppDataObject;
use glib::object::ObjectExt;
use glib::subclass::prelude::{ObjectImpl, ObjectSubclass};
use gtk::subclass::prelude::{
    BoxImpl, CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl,
};
use gtk::subclass::widget::WidgetClassExt;
use gtk::{CompositeTemplate, TemplateChild};

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/zhoujing/jz-launcher/ui/search-result-item.ui")]
pub struct SearchResultItem {
    // 应用图标控件
    #[template_child]
    pub icon: TemplateChild<gtk::Image>,
    // 应用名称标签控件
    #[template_child]
    pub name_label: TemplateChild<gtk::Label>,
    // 应用说明标签控件
    #[template_child]
    pub comment_label: TemplateChild<gtk::Label>,
}

#[glib::object_subclass]
impl ObjectSubclass for SearchResultItem {
    const NAME: &'static str = "MySearchResultItem";
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
    /// 绑定属性
    pub(super) fn bind(&self, app_data_obj: Option<AppDataObject>) {
        if let Some(app_data) = app_data_obj {
            app_data.bind_property("name", &self.name_label.get(), "label")
                .sync_create()
                .build();
            app_data.bind_property("comment", &self.comment_label.get(), "label")
                .sync_create()
                .build();
            if let Some(icon) = app_data.icon() {
                let icon = if icon.contains('/') {
                    "file"
                } else {
                    "icon-name"
                };
                app_data.bind_property("icon", &self.icon.get(), icon)
                    .sync_create()
                    .build();
            }
        }
    }
}
