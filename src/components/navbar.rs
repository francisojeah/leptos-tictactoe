use leptos::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="flex justify-center items-center w-full">
            <nav class="navbar">
                <ul>
                    <li><a href="/"> "Tic-Tac-Toe" </a></li>
                    <li><a href="/about"> "About Us" </a></li>
                </ul>
            </nav>
        </div>
    }
}
