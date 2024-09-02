use yew::prelude::*;

mod card;
mod game;

use game::Game;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Game />
    }
}

fn main() {
    yew::Renderer::<Game>::new().render();
}