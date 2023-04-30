use leptos::{component, view, IntoView, Scope};

#[component]
pub fn NotFound(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="w-full h-full bg-red-300">
            <h1 class="text-3xl">"Page Not Found"</h1>
        </div>
    }
}