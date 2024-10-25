use crate::tictactoe::{GameState, Player};
use leptos::*;

#[component]
pub fn TicTacToeBoard(
    on_cell_click: impl Fn(usize) + Clone + 'static,
    state: Signal<GameState>,
) -> impl IntoView {
    let cells = move || {
        let on_cell_click = on_cell_click.clone();
        (0..9)
            .map(move |i| {
                let on_click = on_cell_click.clone();
                let mark = state.get().board[i].map_or_else(
                    || "".to_string(),
                    |p| match p {
                        Player::X => "X".to_string(),
                        Player::O => "O".to_string(),
                    },
                );

                // Check if this cell is part of the winning combination
                let is_winning_cell = state
                    .get()
                    .winning_cells
                    .map(|cells| cells.contains(&i))
                    .unwrap_or(false);

                // Add winning-cell class if this is part of winning combination
                let cell_class = if is_winning_cell {
                    "cell winning-cell"
                } else {
                    "cell"
                };

                view! {
                    <div
                        class={cell_class}
                        on:click=move |_| on_click(i)
                    >
                        { mark }
                    </div>
                }
            })
            .collect_view()
    };

    view! {
        <div class="board">
        <style>
            ".board .winning-cell {
                background-color: #90EE90 !important;
                transition: background-color 0.3s ease;
            }"
        </style>
            { cells }
        </div>
    }
}
