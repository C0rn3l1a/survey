use leptos::{component, view, IntoView, Scope};

#[component]
pub fn Contact(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="w-full h-full bg-green-300">
            <h1 class="text-3xl">"CONTACT"</h1>
        </div>
    }
}