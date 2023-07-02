use leptos::*;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    let double_count = move || count() * 2;

    view! { cx,
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
            class:red=move || count() % 2 == 1
        >
            "click me: "
            {count}
        </button>

        <br/>
        <ProgressBar progress=count/>
        
        <br/>

        <ProgressBar progress=Signal::derive(cx, double_count)/> 
        <p>"count: " {count}</p>
        <p>"double_count: " {double_count}</p>

    }
}

#[component]
fn ProgressBar(
    cx: Scope,
    #[prop(default=100)]
    max: u16,
    #[prop(into)]
    progress:Signal<i32>
) -> impl IntoView {
    view! {  cx,
        <progress
            max=max
            value=progress
        />
    }
}
    
