use gloo::console::Timer;
use yew::prelude::*;
use gloo_timers::callback::Timeout;
use rand::prelude::*;
use std::{collections::VecDeque, ops::Div};
use std::time::Duration;

use crate::card::{Card, create_deck};

pub enum Msg {
    Start,
    Pause,
    Cancel,
    NextCard,
    Tick,
} 

pub struct Game {
    current_card: Option<Card>,
    deck: VecDeque<Card>,
    is_running: bool,
    timer: Option<Timeout>,
    display_duration: Duration,
}


impl Component for Game {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut deck = create_deck();
        deck.make_contiguous().shuffle(&mut thread_rng());
        Self { 
            current_card: None, 
            deck, 
            is_running: false, 
            timer: None, 
            display_duration: Duration::from_secs(10),
         }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Start => {
                self.is_running = true;
                self.select_next_card();
                self.schedule_tick(ctx);
            }
            Msg::Pause => {
                self.is_running = false;
                self.timer = None;
            }
            Msg::Cancel => {
                self.is_running = false;
                self.timer = None;
                self.current_card = None;
            }
            Msg::NextCard => {
                self.select_next_card();
                self.schedule_tick(ctx);
            }
            Msg::Tick => {
                if self.is_running {
                    self.select_next_card();
                    self.schedule_tick(ctx);
                }
            }
        } 
    true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html!{
            <div>
                <h1>{ "Card Game" }</h1>
                <div>
                    <button onclick={link.callback(|_| Msg::Start)} disabled={self.is_running}>
                        { "Start" }
                    </button>
                    <button onclick={link.callback(|_| Msg::Pause)} disabled={!self.is_running}>
                        { "Pause" }
                    </button>
                    <button onclick={link.callback(|_| Msg::Cancel)}>
                        { "Stop" }
                    </button>
                </div>
                <div>
                    <h2>{ "Current Card" }</h2>
                    { self.render_card() }
                </div>
            </div>
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
        let duration_ms = self.display_duration.as_millis() as u32;
        let timer = Timeout::new(duration_ms, move || link.send_message(Msg::Tick));
        self.timer = Some(timer);
    }

    fn render_card(&self) -> Html {
        match &self.current_card {
            Some(card) => {
                html! {
                    <div>
                        <p>{ &card.name }</p>
                        <img src={card.image.clone()} alt={card.name.clone()} />
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


