use leptos::{component, view, IntoView, Scope};
use leptos_router::*;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! { cx,
        <header class="absolute inset-x-0 top-0 z-50">
            <nav class="flex items-center justify-between p-6 lg:px-8" aria-label="Global">
                <div class="flex items-center gap-4">
                    <img data-trunk src="/static/images/poi.svg" class="h-8 w-auto" alt="Logo"/>
                    <span>"Sample Website"</span>
                </div>

                <div class="flex items-center gap-8">
                    <A exact=true href="/">"Home"</A>
                    <A href="/about-us">"about-us"</A>
                    <A href="/contact">"contact"</A>
                    <A href="/survey">"survey"</A>
                </div>

                <div class="flex items-center gap-4">
                    <button class="bg-orange-400 px-4 py-2 rounded">"Explore"</button>
                </div>
            </nav>
        </header>
    }
}