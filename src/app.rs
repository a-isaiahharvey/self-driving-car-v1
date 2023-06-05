use gloo::{
    console::console_dbg,
    render::{request_animation_frame, AnimationFrame},
    utils::{document, window},
};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::{html, Component};

use crate::{
    car::{Car, CarPtr},
    controls::ControlKind,
    road::Road,
};

#[derive(Debug, Default)]
pub enum Msg {
    #[default]
    None,
    AnimationFrame(f64),
}

#[derive(Debug, Default)]
pub struct App {
    car_canvas: Option<HtmlCanvasElement>,
    car_ctx: Option<CanvasRenderingContext2d>,
    car: CarPtr,
    traffic: Vec<CarPtr>,
    road: Road,
    animation: Option<AnimationFrame>,
}

impl App {
    fn animate(&mut self, _: f64) {
        for i in 0..self.traffic.len() {
            self.traffic[i].update(&self.road.borders, &Vec::new());

            // console_dbg!(self.traffic[i]);
        }
        self.car.update(&self.road.borders, &self.traffic);

        if let Some(canvas) = &self.car_canvas {
            canvas.set_height(window().inner_height().unwrap().as_f64().unwrap() as u32);

            if let Some(ctx) = &self.car_ctx {
                ctx.save();
                ctx.translate(0.0, -self.car.y + canvas.height() as f64 * 0.7)
                    .unwrap();
                self.road.draw(ctx);
                for i in 0..self.traffic.len() {
                    self.traffic[i].draw(ctx, "red");
                }

                self.car.draw(ctx, "blue");

                ctx.restore();
            }
        }
    }
}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(_: &yew::Context<Self>) -> Self {
        Self::default()
    }

    fn view(&self, _: &yew::Context<Self>) -> yew::Html {
        html! {
            <>
                <canvas id="myCanvas"></canvas>
            </>
        }
    }

    fn update(&mut self, _: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::None => (),
            Msg::AnimationFrame(time) => self.animate(time),
        }
        true
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, first_render: bool) {
        if first_render {
            self.car_canvas = Some(
                document()
                    .get_element_by_id("myCanvas")
                    .unwrap()
                    .dyn_into::<HtmlCanvasElement>()
                    .unwrap(),
            );
            if let Some(car_canvas) = &self.car_canvas {
                car_canvas.set_width(200);
                self.car_ctx = Some(
                    car_canvas
                        .get_context("2d")
                        .unwrap()
                        .unwrap()
                        .dyn_into::<CanvasRenderingContext2d>()
                        .unwrap(),
                );

                self.road = Road::new(
                    car_canvas.width() as f64 / 2.0,
                    car_canvas.width() as f64 * 0.9,
                    None,
                );
            }
            self.car = Car::new(
                self.road.get_late_center(1),
                100.0,
                30.0,
                50.0,
                ControlKind::Keys,
                None,
            );

            self.traffic = vec![Car::new(
                self.road.get_late_center(1),
                -100.0,
                30.0,
                50.0,
                ControlKind::Dummy,
                Some(2.0),
            )];
        }

        {
            let link = ctx.link().clone();
            self.animation = Some(request_animation_frame(move |timestamp| {
                link.send_message(Msg::AnimationFrame(timestamp));
            }));
        }
    }
}
