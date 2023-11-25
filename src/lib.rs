#![feature(fn_traits, let_chains)]
#![allow(dead_code, unused_imports, unused_variables)]

pub mod app;
pub mod util;

mod action;
mod component;
mod components;
mod event;
mod identifier;
mod tui;
mod view;

use app::App;

use crate::view::View;

pub struct Context<'a> {
    view: &'a mut View,
}

impl<'a> Context<'a> {
    pub fn new(view: &'a mut View) -> Self {
        Self { view }
    }
}

impl From<&'static mut App> for Context<'_> {
    fn from(app: &'static mut App) -> Self {
        Self {
            view: &mut app.view,
        }
    }
}
