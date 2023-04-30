use leptos::*;
use leptos_router::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="w-full h-full bg-yellow-300">
            <h1 class="text-3xl">"HOME"</h1>
        </div>
    }
}