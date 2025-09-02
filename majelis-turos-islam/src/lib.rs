use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, StaticSegment};

// Modules
mod app;
mod components;
mod layouts;
mod pages;

// Top-Level pages
use crate::{
    layouts::main_layout::MainLayout,
    pages::{
        about::About, article::Article, fatwa::Fatwa, home::Home, not_found::NotFound,
        tasawuf::Tasawuf,
    },
};

pub const BACKEND_URL: &str = "http://127.0.0.1:8787";

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        // sets the document title
        <Title text="Majelis Turos Islam" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <Stylesheet
            href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.13.1/font/bootstrap-icons.css"
            id="bootstrap-icons"
        />

        <Router>
            <Routes fallback=|| view! { <NotFound /> }>
                <ParentRoute path=leptos_router::path!("/") view=MainLayout>
                    <Route path=StaticSegment("") view=Home />
                    <Route path=StaticSegment("artikel") view=Article />
                    <Route path=StaticSegment("fatwa-maklumat") view=Fatwa />
                    <Route path=StaticSegment("tawawuf") view=Tasawuf />
                    <Route path=StaticSegment("tentang") view=About />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
