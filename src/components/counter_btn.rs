use leptos::prelude::*;

/// A parameterized incrementing button
#[component]
pub fn Button(#[prop(default = 1)] increment: i32) -> impl IntoView {
    let count = RwSignal::new(0);

    view! {
        <button
            class="btn"
            on:click=move |_| {
                count.update(|count| *count += increment);
            }>
            "Click me: " {count}
        </button>
    }
}
