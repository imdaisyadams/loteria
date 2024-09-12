use yew::prelude::*;
use gloo_timers::callback::Interval;
use rand::prelude::*;
use std::collections::VecDeque;
use web_sys::HtmlInputElement;

use crate::card::{Card, create_deck};

struct Winner {
    prize: String,
    name: String,
}
pub enum Msg {
    Start,
    Pause,
    Resume,
    Cancel,
    NextCard,
    Tick,
    AddWinner(usize, String),
} 

pub struct Game {
    current_card: Option<Card>,
    deck: VecDeque<Card>,
    is_running: bool,
    timer: Option<Interval>,
    display_duration: u32,
    remaining_time: u32,
    winners: Vec<Winner>,
}


impl Component for Game {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut deck = create_deck();
        let display_duration = 5;
        deck.make_contiguous().shuffle(&mut thread_rng());
        Self { 
            current_card: None, 
            deck, 
            is_running: false, 
            timer: None, 
            display_duration,
            remaining_time: display_duration,
            winners: vec![
                Winner { prize: "Four Corners".to_string(), name: String::new() },
                Winner { prize: "Diagonal".to_string(), name: String::new() },
                Winner { prize: "Row".to_string(), name: String::new() },
                Winner { prize: "Column".to_string(), name: String::new() },
            ],
         }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Start => {
                self.is_running = true;
                if self.current_card.is_none() {
                    self.select_next_card();
                }
                self.remaining_time = self.display_duration;
                self.schedule_tick(ctx);
            }
            Msg::Pause => {
                self.is_running = false;
                self.timer = None;
            }
            Msg::Resume => {
                self.is_running = true;
                self.schedule_tick(ctx);
            }
            Msg::Cancel => {
                self.is_running = false;
                self.timer = None;
                self.current_card = None;
                self.remaining_time = self.display_duration;
            }
            Msg::NextCard => {
                self.select_next_card();
                self.remaining_time = self.display_duration;
            }
            Msg::Tick => {
                if self.is_running {
                    if self.remaining_time > 0 {
                        self.remaining_time -= 1;
                    } else {
                        self.select_next_card();
                        self.remaining_time = self.display_duration;
                    }
                    self.schedule_tick(ctx);
                }
            }
            Msg::AddWinner(index, name) => {
                if let Some(winner) = self.winners.get_mut(index) {
                    winner.name = name;
                }
            }
        } 
    true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html!{
            <>
            <div class="container text-center">
                <div class="col p-3">
                    // title
                    <h1>{ "¿Juguemos Loteria?" }</h1>

                    // render card
                    <h2>{ "Current Card" }</h2>
                    { self.render_card() }
                    <p> { self.remaining_time } </p>

                    // play buttons
                    <button type="button" class="btn btn-primary m-1" onclick={link.callback(|_| Msg::Start)} disabled={self.is_running}>
                        { "Start" }
                    </button>
                    <button type="button" class="btn btn-danger m-1" onclick={link.callback(|_| Msg::Cancel)}>
                        { "End" }
                    </button>
                    if self.is_running {
                        <button type="button" class="btn btn-warning m-1" onclick={link.callback(|_| Msg::Pause)}>
                            { "Pause" }
                        </button>
                        // If game is not running and there is a current card show resume button
                    } else if self.current_card.is_some() {
                        <button type="button" class="btn btn-warning m-1" onclick={link.callback(|_| Msg::Resume)}>
                            { "Resume" }
                        </button>
                    }
                </div>
            </div>
            <div class="container text-left">
                <div class="prize-list">
                    <h2>{ "Prizes" }</h2>
                    { for self.winners.iter().enumerate().map(|(index, prize)| {
                        let index_clone = index;
                        html! {
                            <div class="prize-entry">
                                <span>{ &prize.prize }</span>
                                <input
                                    type="text"
                                    placeholder="Enter winner's name"
                                    value={prize.name.clone()}
                                    oninput={link.callback(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        Msg::AddWinner(index_clone, input.value())
                                    })}
                                />
                            </div>
                        }
                    }) }
                </div>
            </div>
            </>
        }
    }
}

impl Game {
    fn select_next_card(&mut self) {
        if let Some(card) = self.deck.pop_front() {
            self.current_card = Some(card.clone());
            self.deck.push_back(card);
        }
    }

    fn schedule_tick(&mut self, ctx: &Context<Self>) {
        let link = ctx.link().clone();
        let interval = Interval::new(1000, move || link.send_message(Msg::Tick));
        self.timer = Some(interval);
    }

    fn render_card(&self) -> Html {
        match &self.current_card {
            Some(card) => {
                html! {
                    <div>
                        <p>{ &card.name }</p>
                        <img src={card.image.clone()} alt={card.name.clone()} />

                        <p> {"test image"} </p>
                        <img src="imgs/gallo.jpg" alt="test img" />
                        <img src="https://www.w3schools.com/images/w3schools_green.jpg" alt="W3Schools.com" />
                    </div>
                }
            }
            None => {
                html! {
                    <p>{ "No card selected" }</p>
                }
            }
        }
    }
}


