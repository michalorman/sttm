use tuikit::{app::{App, Action}, error::Result, widget::{header::Header, footer::Footer, tabs::{Tabs, Tab}}, scene::Scene};

fn main() -> Result<()> {
    let mut app = App::default();

    let mut header = Header::new("sttm");

    let mut footer = Footer::new();
    footer
        .add_hotkey("tab", "switch")
        .add_hotkey("j", "down")
        .add_hotkey("k", "up");

    let mut tabs = Tabs::new();
    tabs
        .add_tab(Tab::new(" Inbox"))
        .add_tab(Tab::new(" Today"))
        .add_tab(Tab::new(" Later"))
        .add_tab(Tab::new(" Anytime"));

    // build scene
    let mut scene = Scene::new();

    scene.add_widget(&mut footer);
    scene.add_widget(&mut header);
    scene.add_widget(&mut tabs);

    // init the app - enter alt screen, hide cursor
    app.init()?;
    app.set_scene(&mut scene);

    loop {
        app.clear_screen()?;
        app.draw()?;
        app.flush()?;

        match app.next_action()? {
            Action::Quit => break,

            _ => ()
        }
    }

    // cleanup app - leave alt screen, show cursor
    app.cleanup()?;

    Ok(())
}
