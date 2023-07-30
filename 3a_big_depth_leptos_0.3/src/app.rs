use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! {cx, <Test/> }
}

#[component]
pub fn Test(cx: Scope) -> impl IntoView {
    view! {cx,
        <A>
            <A>
                <A>
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
                </A>
            </A>
        </A>
    }
}

#[component]
pub fn A(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <div>
            "A"
            {children(cx)}
        </div>
    }
}
