// glib and other dependencies are re-exported by the gtk crate
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Grid};

// When the application is launched…
fn on_activate(application: &gtk::Application) {
    // Create a new window
        let window = ApplicationWindow::builder()
            .application(application)
            .title("GTK Grid Example")
            .default_width(300)
            .default_height(200)
            .build();

        // Create a new grid
        let grid = Grid::builder()
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .row_spacing(12)
            .column_spacing(12)
            .halign(gtk::Align::Center)  // Horizontal alignment
            .valign(gtk::Align::Center)  // Vertical alignment
            .build();

        // Create some buttons to put in the grid
        let button1 = Button::with_label("Button 1");
        let button2 = Button::with_label("Button 2");
        let button3 = Button::with_label("Button 3");
        let button4 = Button::with_label("Button 4");

        // Attach buttons to the grid
        // Parameters: child, left, top, width, height
        grid.attach(&button1, 0, 0, 1, 1);
        grid.attach(&button2, 0, 1, 1, 1);
        grid.attach(&button3, 1, 0, 2, 1);
        grid.attach(&button4, 0, 2, 1, 2);

        // Add the grid to the window
        window.set_child(Some(&grid));
        
        window.present();
}

fn main() {
    // Create a new application with the builder pattern
    let app = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();
    app.connect_activate(on_activate);
    // Run the application
    app.run();
}
