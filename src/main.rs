use leptos::{html::*, prelude::*};

fn main() {
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Header />

        <main>
            <Intro />
        </main>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <header>
            <hgroup>
                <h1>Alasdair Cooper</h1>
                <p>.NET Web Developer</p>
            </hgroup>
        </header>
    }
}

#[component]
fn Intro() -> impl IntoView {
    view! {
        <section>
            <p>
                Hi, welcome to my website. My name is Alasdair and I am a .NET web developer based in Walsall, UK. This site contains a breakdown of my professional roles and hobby projects by technology used.
            </p>

            <h2>How to Use This Site</h2>

            <p>Click on a tag or technology to filter by it.</p>
        </section>
    }
}
