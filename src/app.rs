use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

const BUTTON_CLASS: &str =
    "px-6 py-2 text-blue-100 bg-blue-500 hover:bg-blue-700 rounded shadow shadow-blue-500/50";
const DIV_COUNTER_CLASS: &str = "mt-4 text-2xl font-bold";

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/test-leptos-axum.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="test" view=TestPage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1 class=DIV_COUNTER_CLASS>"Welcome to Leptos!"</h1>
        <button on:click=on_click class=BUTTON_CLASS>"Click Me: " {count}</button>
        <div>
            <A href="test">Test page</A>
        </div>
    }
}

/// Renders the test page of your application.
#[component]
fn TestPage() -> impl IntoView {
    let navigate = use_navigate();
    view! {
        <h1 class=DIV_COUNTER_CLASS>"Welcome to test page!"</h1>

        <button on:click=move |_| navigate("", Default::default()) class=BUTTON_CLASS>Home</button>
    }
}
