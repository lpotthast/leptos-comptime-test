use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! { <Test/> }
}

#[component]
pub fn Test() -> impl IntoView {
    let siblings = view! {
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
    };

    view! {
        <A>
            <A>
                <A>
                    <A>
                        <A>
                            <A>
                                {siblings}
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
    view! {
        <div>
            "A"
            {children()}
        </div>
    }
}
