use leptos::*;
use leptos_router::*;
use crate::components::navbar::Navbar;
use crate::about::AboutUs;
use crate::tictactoe::TicTacToe;

mod components;
mod tictactoe; 
mod about; 

fn main() {
    mount_to_body(|| view! {
        <div>
            <Navbar/>
            <Router>
                <Routes>
                    <Route path="/" view=|| view! { <TicTacToe/> } />
                    <Route path="/about" view=|| view! { <AboutUs/> } />
                </Routes>
            </Router>
        </div>
    });
}
