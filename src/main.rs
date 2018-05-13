extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use gtk::{
    Button,
    ButtonExt,
    ContainerExt,
    Inhibit,
    Label,
    LabelExt,
    WidgetExt,
    Window,
    WindowType,
};
use gtk::Orientation::Vertical;
use relm::{Relm, Update, Widget};

struct Model {
    counter: i32
}

#[derive(Msg)]
enum Msg {
    Increment,
    Decrement,
    Quit,
}

struct Win {
    model: Model,
    widgets: Widgets,
}

// Create the structure that holds the widgets used in the view.
#[derive(Clone)]
struct Widgets {
    counter_label: Label,
    minus_button: Button,
    plus_button: Button,
    window: Window,
}

impl Update for Win {
    type Model = Model;
    type ModelParam = ();

    type Msg = Msg;

    // Return the initial model.
    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {
            counter: 0,
        }
    }

    // The model may be updated when a message is received.
    // Widgets may also be updated in this function.
    // Futures and streams can be connected to send a message when a value is ready.
    fn update(&mut self, event: Msg) {
        let label = &self.widgets.counter_label;

        match event {
            Msg::Increment => {
                self.model.counter += 1;
                label.set_text(&self.model.counter.to_string());
            },
            Msg::Decrement => {
                self.model.counter -= 1;
                label.set_text(&self.model.counter.to_string());
            },
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    type Root = Window;

    // Return the root widget.
    fn root(&self) -> Self::Root {
        self.widgets.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
         // Create the view using the normal GTK+ method calls.
        let vbox = gtk::Box::new(Vertical, 0);

        // Increment
        let plus_button = Button::new_with_label("+");
        vbox.add(&plus_button);

        // Decrement
        let minus_button = Button::new_with_label("-");
        vbox.add(&minus_button);

        // Counter Label
        let counter_label = Label::new("0");
        vbox.add(&counter_label);

        // New window
        let window = Window::new(WindowType::Toplevel);

        // add view
        window.add(&vbox);
        window.show_all();

        // Send the message Increment when the button is clicked.
        connect!(relm, plus_button, connect_clicked(_), Msg::Increment);
        connect!(relm, minus_button, connect_clicked(_), Msg::Decrement);
        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));

        Win {
            model,
            widgets: Widgets {
                counter_label,
                minus_button,
                plus_button,
                window: window,
            },
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
