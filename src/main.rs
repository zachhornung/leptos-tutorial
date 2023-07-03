use leptos::{*, ev::MouseEvent};

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
        <DynamicList initial_length=1 />

    }
}

/// displays progress of some action or process
#[component]
fn ProgressBar(
    cx: Scope,
    /// The max value of the progress bar
    #[prop(default=100)]
    max: u16,
    /// How much progress the process has made thus far
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
    
#[component]
fn DynamicList(
    cx: Scope,
    initial_length: usize,
) -> impl IntoView {
    let mut next_counter_id = initial_length;
    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(cx, id + 1)))
        .collect::<Vec<_>>();
    let (counters, set_counters) = create_signal(cx, initial_counters);
    let add_counter = move |_| {
        let sig = create_signal(cx, next_counter_id + 1);
        set_counters.update(move |counters| {
            counters.push((next_counter_id, sig))
        });
        next_counter_id += 1;
    };

    view! { cx,
        <div>
            <button on:click=add_counter>
                "Add Counter" 
            </button>
            <ul>
                <For
                    each=counters
                    key=|counter| counter.0
                    view=move |cx, (id, (count, set_count))| {
                        view! { cx,
                            <li>
                                <button on:click=move |_| {set_count.update(|n| *n += 1)}>
                                    {count}
                                </button>
                                <button
                                    on:click=move |_| {
                                        set_counters.update(|counters| {
                                            counters.retain(|(counter_id, _)| counter_id != &id)
                                        });
                                    } 
                                >
                                    "Remove" 
                                </button> 
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }

}


