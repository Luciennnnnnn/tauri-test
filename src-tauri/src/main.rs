#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
use tauri::api::dialog;

use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

fn main() {

  // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
  let quit = CustomMenuItem::new("open".to_string(), "Open...");
  let close = CustomMenuItem::new("open_folder".to_string(), "Open Folder...");

  let xx = CustomMenuItem::new("xx".to_string(), "xx");

  let submenu1 = Submenu::new("app", Menu::new().add_native_item(MenuItem::Copy).add_native_item(MenuItem::Hide));

  let submenu2 = Submenu::new("File", Menu::new().add_item(quit).add_item(close));

  let submenu3 = Submenu::new("Edit", Menu::new().add_item(xx));

  let menu = Menu::new()
  .add_submenu(submenu1)
  .add_submenu(submenu2)
  .add_submenu(submenu3);

  tauri::Builder::default()
    .menu(menu)
    .on_menu_event(|event| {
      match event.menu_item_id() {
        "open" => {
          dialog::FileDialogBuilder::new()
            .add_filter("Image", &["jpg", "png"])
            .pick_file(move |file_path| match file_path {
              Some(p) => {
                // let img = image::open(p).unwrap();
                event.window().emit("display-image", p).unwrap();
              }
              None => {}
              // do something with the optional file paths here
              // the file paths value is `None` if the user closed the dialog
            })
        }
        "open_folder" => {
          dialog::FileDialogBuilder::new()
            .add_filter("Image", &["jpg", "png"])
            .pick_folder(|folder_path| {
              // do something with the optional file paths here
              // the file paths value is `None` if the user closed the dialog
            })
        }
        _ => {}
      }
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
