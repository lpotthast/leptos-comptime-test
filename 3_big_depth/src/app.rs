use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! { <Test/> }
}

#[component]
pub fn Test() -> impl IntoView {
    view! {
        <A>
            <A>
                <A>
                    <A>
                        <A>
                            <A>
                                <A>
                                    <A>
                                        <A>
                                            <A>"Siblingf"</A>
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
pub fn A(children: Children) -> impl IntoView {
    view! {
        <div>
            "A"
            {children()}
        </div>
    }
}
