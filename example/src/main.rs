// A proof of concept greeter. This does not have any actual login ability, it just exists.
use lightdm::*;
use glib::MainLoop;
use lightdm::auto::GreeterExt;

fn show_prompt_cb(_greeter: &lightdm::Greeter, _text: &str, _t: lightdm::PromptType) {
    println!("Show prompt callback");
}

fn authentication_complete_cb(_greeter: &lightdm::Greeter) {
    println!("Auth complete callback");
}

fn main() {
    println!("Hello, world!");

    let main_loop: MainLoop = MainLoop::new(None, false);

    let _ = gtk4::init();

    let text = gtk4::Label::builder()
        .label("Hello from Rust!").build();

    let _window = gtk4::ApplicationWindow::builder()
        .default_height(500)  // random value
        .default_height(30)
        .visible(true)
        .child(&text)
        .build();

    let greeter: Greeter = Greeter::new();
    greeter.connect_show_prompt(show_prompt_cb);
    greeter.connect_authentication_complete(authentication_complete_cb);

    let sync = greeter.connect_to_daemon_sync();
    if sync.is_ok() {
        println!("Connect sync OK");
    } else if sync.is_err() {
        panic!("Err! {:#?}", sync.err());
    }

    let _ = greeter.authenticate(None);

    main_loop.run();

    println!("TODO: Play Bad Apple.");
}
