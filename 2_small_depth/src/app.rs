use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let (show, set_show) = create_signal(false);
    let toggle = move |_| match show.get_untracked() {
        true => {
            tracing::info!("hide <Test>");
            set_show.set(false);
        }
        false => {
            tracing::info!("show <Test>");
            set_show.set(true)
        }
    };
    view! {
        <button on:click=toggle>"Toggle component"</button>
        { move || show.get().then(|| view! { <Test/> }) }
    }
}

#[component]
pub fn Test() -> impl IntoView {
    tracing::info!("initialize <Test>");
    view! {
        <A>
            <A>
                <A>
                    <A>
                        <A>
                            <A>
                                <A>"Sibling"</A>
                            </A>
                        </A>
                    </A>
                </A>
            </A>
        </A>
    }
}

#[component]
pub fn A(children: Children) -> impl IntoView {
    tracing::info!("initialize <A>");
    view! {
        <div>
            "A"
            {children()}
        </div>
    }
}
