use leptos::*;
use crate::tictactoe::{GameState, Player};

#[component]
pub fn TicTacToeBoard(
    on_cell_click: impl Fn(usize) + Clone + 'static,
    state: Signal<GameState>,
) -> impl IntoView {
    let cells = move || {
        let on_cell_click = on_cell_click.clone();
        (0..9).map(move |i| {
            let on_click = on_cell_click.clone();
            let mark = state.get().board[i].map_or_else(|| "".to_string(), |p| match p {
                Player::X => "X".to_string(),
                Player::O => "O".to_string(),
            });

            view! {
                <div 
                    class="cell" 
                    on:click=move |_| on_click(i)
                >
                    { mark }
                </div>
            }
        }).collect_view()
    };

    view! {
        <div class="board">
            { cells }
        </div>
    }
}