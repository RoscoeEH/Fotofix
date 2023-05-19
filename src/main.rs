extern crate gtk;
use gtk::prelude::*;
use gtk::{Button, Grid, Window, WindowType};

use std::path::{Path, PathBuf};

fn choose_file() -> Option<std::path::PathBuf> {
    None
}

fn main() {
    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Create a new top-level window
    let window = Window::new(WindowType::Toplevel);

    // Set the title and default size of the window
    window.set_title("FotoFix");
    window.set_default_size(1200, 900);

    // Handle the window's delete event
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Create button grid
    let grid = Grid::new();
    grid.set_row_spacing(5);
    grid.set_column_spacing(5);
    window.add(&grid);

    // Create a button and add it to the grid
    let loadButton = Button::with_label("Load");
    grid.add(&loadButton);

    loadButton.connect_clicked(|loadButton| {
        println!("Load");
    });

    // Create a button and add it to the grid
    let brightnessButton = Button::with_label("Brightness");
    grid.add(&brightnessButton);

    brightnessButton.connect_clicked(|brightnessButton| {
        println!("Bright");
    });

    // Create a button and add it to the grid
    let contrastButton = Button::with_label("Contrast");
    grid.add(&contrastButton);

    contrastButton.connect_clicked(|contrastButton| {
        println!("Contrast");
    });

    let burnButton = Button::with_label("Burn");
    grid.add(&burnButton);

    burnButton.connect_clicked(|burnButton| {
        println!("Burn");
    });

    let dodgeButton = Button::with_label("Dodge");
    grid.add(&dodgeButton);

    dodgeButton.connect_clicked(|dodgeButton| {
        println!("Dodge");
    });

    let filterButton = Button::with_label("Filter");
    grid.add(&filterButton);

    filterButton.connect_clicked(|filterButton| {
        println!("Filter");
    });

    // Show all widgets
    window.show_all();

    // Start the GTK main event loop
    gtk::main();
}
