use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (show, set_show) = create_signal(cx, false);
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
    view! {cx,
        <button on:click=toggle>"Toggle component"</button>
        { move || show.get().then(|| view! {cx, <Test/> }) }
    }
}

#[component]
pub fn Test(cx: Scope) -> impl IntoView {
    tracing::info!("initialize <Test>");
    view! {cx,
        <A>
            <A>
                <A>
                    <A>
                        <A>
                            <A>
                                <A>"Sibling"</A>
                                <A>"Sibling"</A>
                                <A>"Sibling"</A>
                                <A>"Sibling"</A>
                                <A>"Sibling"</A>
                                <A>"Sibling"</A>
                                <A>"Sibling"</A>

                                <A>"... even more siblings here!"</A>
                                <A>"Sibling"</A>
                                <A>"Sibling"</A>
                                <A>"Sibling"</A>
                                <A>"Sibling"</A>
                                <A>"Sibling"</A>
                                <A>"Sibling"</A>
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
pub fn A(cx: Scope, children: Children) -> impl IntoView {
    tracing::info!("initialize <A>");
    view! {cx,
        <div>
            "A"
            {children(cx)}
        </div>
    }
}
