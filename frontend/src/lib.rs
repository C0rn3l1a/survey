use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod views;

use views::home::*;
use views::about_us::*;
use views::contact::*;
use views::survey::*;
use views::not_found::*;

mod components;

use components::header::*;


#[component]
pub fn BaseLayout(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    view! { cx,
        <Router>
            <Header/>
            <main class="mt-20">
                <Routes>
                    <Route
                        path=""
                        view=move |cx| view! { cx, <Home/> }
                    />
                    <Route
                        path="about-us"
                        view=move |cx| view! { cx, <AboutUs/> }
                    />
                    <Route
                        path="contact"
                        view=move |cx| view! { cx, <Contact/> }
                    />
                    <Route
                        path="survey"
                        view=move |cx| view! { cx, <Survey/> }
                    />
                    <Route
                        path="*"
                        view=move |cx| view! { cx,  <NotFound/> }
                    />
                </Routes>
            </main>
        </Router>
    }
}
