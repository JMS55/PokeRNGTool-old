#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};
use gtk::{
    Application, ApplicationWindow, Button, ContainerExt, GtkWindowExt, HeaderBar, HeaderBarExt,
    Settings, SettingsExt, Stack, StackExt, StackSwitcher, StackSwitcherExt, StackTransitionType,
    StyleContextExt, WidgetExt, STYLE_CLASS_SUGGESTED_ACTION,
};
use std::env::args;

mod lcrng;
mod misc;
mod stationary;
mod widgets;

fn main() {
    let mut application_flags = ApplicationFlags::empty();
    application_flags.insert(ApplicationFlags::NON_UNIQUE);
    let application = Application::new("org.jms.PokeRNGTool", application_flags)
        .expect("Failed to initialize gtk::Application");

    application.connect_startup(|_| {
        Settings::get_default()
            .unwrap()
            .set_property_gtk_application_prefer_dark_theme(true);
    });

    application.connect_activate(|application| {
        let search_button = Button::new_with_label("Search");
        search_button
            .get_style_context()
            .add_class(&STYLE_CLASS_SUGGESTED_ACTION);
        let reset_button = Button::new_with_label("Reset");

        let stack = Stack::new();
        stack.set_transition_type(StackTransitionType::OverUpDown);
        stack.add_titled(
            &stationary::ui(&search_button, &reset_button, &stack),
            "stationary",
            "Stationary",
        );

        let stack_switcher = StackSwitcher::new();
        stack_switcher.set_stack(Some(&stack));

        let headerbar = HeaderBar::new();
        headerbar.set_title("PokeRNGTool");
        headerbar.set_custom_title(&stack_switcher);
        headerbar.set_show_close_button(true);
        headerbar.add(&search_button);
        headerbar.add(&reset_button);

        let window = ApplicationWindow::new(application);
        window.set_icon_name("org.jms.PokeRNGTool");
        window.set_titlebar(&headerbar);
        window.set_border_width(18);
        window.set_default_size(1000, 580);
        window.connect_delete_event(move |window, _| {
            window.destroy();
            gtk::Inhibit(false)
        });
        window.add(&stack);
        window.show_all();
    });

    application.run(&args().collect::<Vec<_>>());
}
