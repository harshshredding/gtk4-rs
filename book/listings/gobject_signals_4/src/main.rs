use glib::clone;
use gtk::{glib, Label, Orientation};
use gtk::{prelude::*, BoxBuilder};
use gtk::{Application, ApplicationWindowBuilder};
use std::{cell::RefCell, env::args};

// Implementation of our custom GObject
mod imp {
    // Import parent scope
    use super::*;
    // Import necessary objects and traits for subclassing
    use glib::{ParamFlags, ParamSpec, Value};
    use gtk::subclass::prelude::*;
    use once_cell::sync::Lazy;

    // Object holding the state
    #[derive(Default)]
    pub struct CustomButton {
        number: RefCell<i32>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for CustomButton {
        const NAME: &'static str = "MyGtkAppCustomButton";
        type Type = super::CustomButton;
        type ParentType = gtk::Button;
    }
    // ANCHOR: object_impl
    // Trait shared by all GObjects
    impl ObjectImpl for CustomButton {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
            obj.set_label(&self.number.borrow().to_string());
        }

        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![ParamSpec::int(
                    // Name
                    "number",
                    // Nickname
                    "number",
                    // Short description
                    "number",
                    // Minimum value
                    i32::MIN,
                    // Maximum value
                    i32::MAX,
                    // Default value
                    0,
                    // The property can be read and written to
                    ParamFlags::READWRITE,
                )]
            });
            PROPERTIES.as_ref()
        }
        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.get_name() {
                "number" => {
                    let input_number = value.get().unwrap().unwrap();
                    self.number.replace(input_number);
                }
                _ => unimplemented!(),
            }
        }

        fn get_property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.get_name() {
                "number" => self.number.borrow().to_value(),
                _ => unimplemented!(),
            }
        }
    }
    // ANCHOR_END: object_impl

    // Trait shared by all widgets
    impl WidgetImpl for CustomButton {}

    // ANCHOR: button_impl
    // Trait shared by all buttons
    impl ButtonImpl for CustomButton {
        fn clicked(&self, button: &Self::Type) {
            let incremented_number = self.number.borrow().clone() + 1;
            button.set_property("number", &incremented_number).unwrap();
            button.set_label(&self.number.borrow().to_string())
        }
    }
    // ANCHOR_END: button_impl
}

glib::wrapper! {
    pub struct CustomButton(ObjectSubclass<imp::CustomButton>)
        @extends gtk::Button, gtk::Widget;
}

impl CustomButton {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create Button")
    }
    pub fn with_label(label: &str) -> Self {
        let button = Self::new();
        button.set_label(label);
        button
    }
}

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default())
        .expect("Initialization failed...");
    app.connect_activate(on_activate);

    // Get command-line arguments
    let args: Vec<String> = args().collect();
    // Run the application
    app.run(&args);
}
// ANCHOR: activate
// When the application is launched…
fn on_activate(application: &Application) {
    // … create a new window …
    let window = ApplicationWindowBuilder::new()
        .application(application)
        .title("My GTK App")
        .build();

    // Create a button
    let button = CustomButton::new();

    // ANCHOR: label
    let label = Label::new(Some("0"));
    button.connect_notify_local(
        Some("number"),
        clone!(@weak label => move |button, _| {
            let number = button.get_property("number").unwrap().get::<i32>().unwrap().unwrap();
            label.set_label(&number.to_string());
        }),
    );
    // ANCHOR_END: label

    // Set up box
    let gtk_box = BoxBuilder::new()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .spacing(12)
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&button);
    gtk_box.append(&label);
    window.set_child(Some(&gtk_box));
    window.present();
}
// ANCHOR_END: activate
