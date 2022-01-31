mod download;
#[cfg(test)]
mod tests;

use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use std::{cell::Cell, rc::Rc};

use download::{download_pack, smart_download};
use glib::clone;
use gtk::gdk::Popup;
use gtk::glib::{MainContext, timeout_future_seconds, PRIORITY_DEFAULT};
use gtk::{prelude::*, NaturalWrapMode, Align, Dialog, DialogFlags, ResponseType};
use gtk::{self, ApplicationWindow, Button, Entry, Label, Orientation, Switch};
use gtk::{glib, Application};

fn main() {
    // Create a new application
    let app = gtk::Application::new(Some("nyc.cleo.rpdl"), Default::default());

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Resource Pack Downloader")
        .build();
    // Create two buttons
    let url_entry = Entry::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    


    // Connect callbacks
    // When a button is clicked, `number` and label of the other button will be changed

    let color_code_switch = Switch::builder()
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(12)
        .margin_end(12)
        .build();

    color_code_switch.set_state(false);
    let color_code_label = Label::builder()
        .label("Filter format codes from downloaded file")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let color_code_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(6)
        .build();
    color_code_box.append(&color_code_switch);
    color_code_box.append(&color_code_label);

    let dl_button = Button::builder()
        .label("Download")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let (sens_sender, sens_receiver) = MainContext::channel(PRIORITY_DEFAULT);
    let (label_sender, label_receiver) = MainContext::channel(PRIORITY_DEFAULT);
    let (err_sender, err_receiver) = MainContext::channel(PRIORITY_DEFAULT);

    let url_entry_clone = url_entry.clone();
    let ccswitch = color_code_switch.clone(); 
    dl_button.connect_clicked( move |_| {
            // The main loop executes the asynchronous block
            // let dl_url_clone = url_entry.text().clone();
            // let color_code_clone = color_code_switch.state().clone(); 
            let dl_url_clone = url_entry_clone.text().clone();
            let color_code_clone = ccswitch.state().clone();
            let sens_sender = sens_sender.clone();
            let label_sender = label_sender.clone();
            let err_sender = err_sender.clone();
            thread::spawn(move || {
                // Deactivate the button until the operation is done
                sens_sender.send(false).expect("Could not send through channel");
                // button.set_sensitive(false);
                label_sender.send("Downloading").expect("Could not send through channel");
                
                match smart_download(dot_minecraft(), &dl_url_clone, color_code_clone) {
                    Ok(_) => {
                        label_sender.send("Downloaded!").expect("Could not send through channel");
                    },
                    Err(e) => {err_sender.send(e.to_string()).expect("Could not send through channel");}
                };
                thread::sleep(Duration::from_secs(1));
                label_sender.send("Download").expect("Could not send through channel");
                // Activate the button again
                sens_sender.send(true).expect("Could not send through channel");

                // button.set_sensitive(true);
            });
        }
    );
    sens_receiver.attach(
        None,
        clone!(@weak dl_button => @default-return Continue(false),
                    move |enable_button| {
                        dl_button.set_sensitive(enable_button);
                        Continue(true)
                    }
        ),
    );
    label_receiver.attach(
        None,
        clone!(@weak dl_button => @default-return Continue(false),
                    move |label| {
                        dl_button.set_label(label);
                        Continue(true)
                    }
        ),
    );
    let window_clone = window.clone();
    err_receiver.attach(
        None,
        clone!(@weak dl_button => @default-return Continue(false),
                    move |err| {
                        error_window(&window_clone, &err);
                        Continue(true)
                    }
        ),
    );
    let window_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    
    window_box.append(&url_entry);
    window_box.append(&dl_button);
    window_box.append(&color_code_box);
    window.set_child(Some(&window_box));
    
    // Add buttons to `gtk_box`
    

    // Create a window
    

    // Present the window
    window.present();
}

fn error_window(parent: &ApplicationWindow, error: &str) {

    let dialog = Dialog::with_buttons(Some("Error"),Some(parent), gtk::DialogFlags::MODAL, &[("Ok", ResponseType::Ok)]);
    // let window = ApplicationWindow::builder()
    //     .application(app)
    //     .title("Error")
    //     .resizable(true)
    //     .build();
    let err_label = Label::builder()
        .label(&format!("Error: {}", error))
        .natural_wrap_mode(NaturalWrapMode::Word)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    dialog.content_area().append(&err_label);
    dialog.connect_response(|d, _| d.close());
    // let window_clone = window.clone();
    // close_button.connect_clicked(move |button| {
    //     window_clone.close();
    // });
 
    // window_box.append(&close_button);
    // window.set_child(Some(&window_box));
    // window.present();
    dialog.present()

    

}

#[cfg(target_os = "linux")]
fn dot_minecraft() -> PathBuf {
    let mut path = PathBuf::new();
    path.push(std::env::var("HOME").unwrap());
    path.push(".minecraft");
    path
     
}
#[cfg(target_os = "macos")]
fn dot_minecraft() -> PathBuf {
    let mut path = PathBuf::new();
    path.push(std::env::var("HOME").unwrap());
    path.push("/Library/Application Support/minecraft");
    path
}
#[cfg(target_os = "windows")]
fn dot_minecraft() -> PathBuf {
    let mut path = PathBuf::new();
    path.push(std::env::var("APPDATA").unwrap());
    path.push(".minecraft");
    path
}