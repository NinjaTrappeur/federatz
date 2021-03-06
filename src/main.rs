use gtk4 as gtk;
use gtk::prelude::*;
use gtk::Application;

mod oauth;
mod ui;
mod db;

use ui::widgets::oauth::create_oauth_assistant;

fn main() {
    let conn = db::open_db();
    match conn  {
        Ok(conn) => db::run_migrations(conn),
        Err(e) => panic!("Error when running the DB migrations: {}", e),
    };
    let app = Application::builder()
        .application_id("fr.alternativebit.federatz")
        .build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let oauth_assistant = create_oauth_assistant(app);
    oauth_assistant.present();
}
