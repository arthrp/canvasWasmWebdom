#![allow(unused_parens)]
use web_dom::*;

#[macro_use]
extern crate ref_thread_local;
use ref_thread_local::RefThreadLocal;

pub struct CanvasState {
    window: Element,
    ctx: DOMReference,
    listener: EventListener,
    square_x: f32,
    square_y: f32,
    square_speed_x: f32,
    square_speed_y: f32,
    width: f32,
    height: f32
}

impl CanvasState {
    pub fn new() -> CanvasState {
        let doc = window::get_document(window());
        let canv = document::query_selector(doc, "#canvas");
        let context = htmlcanvas::get_context(canv, "2d");

        return CanvasState { window: window(), ctx: context, listener: create_event_listener(), square_speed_x: 1.0, square_speed_y: 1.0, square_x: 0.0, 
            square_y: 0.0, width: 600.0, height: 400.0 };
    }

    pub fn init(&self) -> () {
        window::request_animation_frame(self.window, self.listener);
        self.draw();
    }

    pub fn move_square(&mut self) -> () {
        if(self.square_x + 50.0 > self.width){
            self.square_speed_x = -1.0;
        }
        else if(self.square_x < 0.0){
            self.square_speed_x = 1.0;
        }

        if(self.square_y + 50.0 > self.height){
            self.square_speed_y = -1.0;
        }
        else if(self.square_y < 0.0){
            self.square_speed_y = 1.0;
        }

        self.square_x += self.square_speed_x;
        self.square_y += self.square_speed_y;

        self.draw();
        window::request_animation_frame(self.window, self.listener);
    }

    fn draw(&self) -> () {
        canvas::clear_rect(self.ctx, 0.0, 0.0, self.width, self.height);
        canvas::fill_rect(self.ctx, self.square_x, self.square_y, 50.0, 50.0);
    }
}

ref_thread_local! {
    static managed CANV_STATE: CanvasState = CanvasState::new();
}

#[no_mangle]
pub fn main() -> () {
    let canv_state = &mut *CANV_STATE.borrow_mut();
    canv_state.init();
}

#[no_mangle]
pub fn callback(_listener: EventListener, _event: Event) {
    let canv_state = &mut *CANV_STATE.borrow_mut();
    canv_state.move_square();
}