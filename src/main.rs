use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <main class="page">
            <header class="header">
                <div class="header-copy">
                    <p class="eyebrow">"LOUTREBASE / 001"</p>
                    <h1>"Liens utiles"</h1>
                    <p class="intro">"Quelques destinations pour commencer."</p>
                </div>
            </header>

            <nav class="destinations" aria-label="Destinations">
                <a class="destination" href="https://www.rust-lang.org/" target="_blank" rel="noopener noreferrer">
                    <span class="number">"01"</span>
                    <span class="destination-copy">
                        <strong>"Rust"</strong>
                        <span>"Le langage et son écosystème"</span>
                    </span>
                    <span class="arrow" aria-hidden="true">"↗"</span>
                </a>
                <a class="destination" href="https://leptos.dev/" target="_blank" rel="noopener noreferrer">
                    <span class="number">"02"</span>
                    <span class="destination-copy">
                        <strong>"Leptos"</strong>
                        <span>"Le framework de cette application"</span>
                    </span>
                    <span class="arrow" aria-hidden="true">"↗"</span>
                </a>
                <a class="destination" href="https://github.com/" target="_blank" rel="noopener noreferrer">
                    <span class="number">"03"</span>
                    <span class="destination-copy">
                        <strong>"GitHub"</strong>
                        <span>"Hébergement et projets"</span>
                    </span>
                    <span class="arrow" aria-hidden="true">"↗"</span>
                </a>
            </nav>

            <footer class="footer">"Un petit point de départ."</footer>
        </main>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
