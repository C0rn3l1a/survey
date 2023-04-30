use leptos::{component, view, IntoView, Scope};

#[component]
pub fn AboutUs(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="w-full h-full bg-red-300">
            <h1 class="text-3xl">"ABOUT US"</h1>
        </div>
    }
}