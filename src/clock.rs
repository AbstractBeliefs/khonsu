use gloo::timers::callback::Interval;
use yew::prelude::*;
use js_sys::Date;
use chrono::{TimeZone, Utc, Duration, Timelike};

#[derive(Properties, PartialEq)]
pub struct ClockProps {
    pub mins_offset: i32,
    pub label: String,
    pub theme: String,
}

pub enum Msg {
    UpdateTime,
}

pub struct Clock {
    time: String,
    _timekeeper: Interval,
}

impl Clock {
    fn get_current_time(clock_offset_mins: i32) -> String {
        let utc_now = Date::new_0();
        let utc_secs = utc_now.get_utc_seconds();
        let utc_mins = utc_now.get_utc_minutes();
        let utc_hours = utc_now.get_utc_hours();

        let utc_now = Utc.with_ymd_and_hms(1970, 1, 1, utc_hours, utc_mins, utc_secs).unwrap();
        let clock_time = utc_now + Duration::minutes(clock_offset_mins.into());

        let clock_hour = clock_time.hour();
        let clock_mins = clock_time.minute();
        let clock_secs = clock_time.second();

        // return time as a string
        format!("{clock_hour:02}:{clock_mins:02}:{clock_secs:02}")
    }
}

impl Component for Clock {
    type Message = Msg;
    type Properties = ClockProps;

    fn create(ctx: &Context<Self>) -> Self {

        let timekeeper_handle = {
            let link = ctx.link().clone();
            Interval::new(300, move || link.send_message(Msg::UpdateTime))
        };

        Self {
            time: Clock::get_current_time(ctx.props().mins_offset),
            _timekeeper: timekeeper_handle,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let props = ctx.props();
        match msg {
            Msg::UpdateTime => {
                self.time = Clock::get_current_time(props.mins_offset);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let style = format!("color: {};", &props.theme);
        html! {
            <div id="time" style={style}>
                { &self.time } { &props.label }
            </div>
        }
    }
}
