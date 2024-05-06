mod chess;

use gtk::{gdk_pixbuf, glib, prelude::*, Picture};
use gtk::{Application, ApplicationWindow, Button, Grid};
use gtk::gdk;
use chess::Board;
use std::time::Duration;

const BOARD_SIZE: usize = 8;
const SQUARE_PIXELS: usize = 60;

fn show_board() {
    gtk::init().expect("Failed to initialize GDK");
    // Initialize GTK
    let app = Application::builder()
        .application_id("it.andrealeonemcs.chessboard")
        .build();

    let display = match gdk::Display::default() {
        Some(display) => display,
        None => {
            eprintln!("Failed to open display");
            return;
        }
    };
    let provider = gtk::CssProvider::new();
    provider.load_from_data(include_str!("../styles/board.css"));
    let priority = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;
    gtk::style_context_add_provider_for_display(&display, &provider, priority);
    // Connect to activate event
    app.connect_activate(|app| {
        // Create a window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Chessboard")
            .default_width((BOARD_SIZE * SQUARE_PIXELS) as i32)
            .default_height((BOARD_SIZE * SQUARE_PIXELS) as i32)
            .build();
        
        // Create a grid to hold the chessboard squares
        let grid = Grid::new();
        grid.set_row_homogeneous(true);
        grid.set_column_homogeneous(true);

        //gtk::StyleContext::add_provider(&grid.style_context(), &provider, priority);

        // Create the chessboard squares
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let button = Button::new();
                button.add_css_class("square");
                button.add_css_class(if (row+col)%2 == 0 {"light-square"} else {"dark-square"});
                if row < 2 && col < 6 {
                    let image_path = format!("images/{}{}.png", row, col);
                    let pixbuf = gdk_pixbuf::Pixbuf::from_file(image_path)
                        .expect("Failed to load image");
                    // Create picture from the sub-image
                    let picture = Picture::new();
                    picture.set_pixbuf(Some(&pixbuf));
                    button.set_child(Some(&picture));
                }
                grid.attach(&button, col as i32, row as i32, 1, 1);
            }
        }
        let mut board = Board::new();
        board.apply_to_grid(&grid);

        // Add the grid to the window
        window.set_child(Some(&grid));

        // Show the window
        window.show();

        // Schedule updates to the UI at regular intervals
        glib::timeout_add_local(Duration::from_secs(1), move || {
            board.play_random_move();
            board.apply_to_grid(&grid);
            glib::ControlFlow::Continue
        });
    });

    // Run the application
    app.run();
}

fn main() {
    show_board();
}