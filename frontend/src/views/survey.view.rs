use leptos::{component, view, IntoView, Scope};

#[component]
pub fn Survey(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="w-full h-full bg-blue-300">
            <h1 class="text-3xl">"SURVEY"</h1>
        </div>
    }
}