use gtk::prelude::*;
use sourceview5::prelude::*;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title(Some("SourceView5 + Rust"));
    window.set_default_size(1280, 800);

    let buffer = sourceview5::Buffer::new(None);
    // buffer.set_highlight_syntax(true);

    let data = std::fs::read("test.txt").unwrap();
    let text = String::from_utf8(data).unwrap();
    buffer.set_text(&text);

    let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    let view = sourceview5::View::with_buffer(&buffer);
    // view.set_monospace(true);
    // view.set_background_pattern(sourceview5::BackgroundPatternType::Grid);
    view.set_show_line_numbers(true);
    // view.set_highlight_current_line(true);
    // view.set_tab_width(4);
    view.set_hexpand(true);
    container.append(&view);
    let map = sourceview5::Map::new();
    map.set_view(&view);
    container.append(&map);

    let sw = gtk::ScrolledWindowBuilder::new()
        .child(&container)
        .build();
    window.set_child(Some(&sw));
    window.show();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.bilelmoussaoui.sourceview5-example"),
        Default::default(),
    );
    application.connect_activate(build_ui);

    application.run();
}
