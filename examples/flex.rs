// Copyright 2024 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

//! Flex properties can be set in Xilem.

use std::time::Duration;

use chrono::{DateTime, Local};
use winit::dpi::LogicalSize;
use winit::error::EventLoopError;
use winit::platform::wayland::Anchor;
use xilem::core::{MessageProxy, fork, lens};
use xilem::masonry::parley::{FontFamily, FontStack, GenericFamily};
use xilem::masonry::properties::types::{AsUnit, Length};
use xilem::style::{Background, Padding, Style};
use xilem::tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use xilem::tokio::time::Instant;
use xilem::view::{
    CrossAxisAlignment, Flex, FlexExt as _, FlexSpacer, Label, MainAxisAlignment, button, flex_row,
    label, sized_box, worker, zstack,
};
use xilem::{EventLoop, FontWeight, WidgetView, WindowOptions, Xilem, palette, tokio};

struct AppState {
    time: DateTime<Local>,
}

fn time_button(time: &mut DateTime<Local>) -> impl WidgetView<DateTime<Local>> + use<> {
    button(
        label(time.format("%b %d  %H:%M").to_string()).weight(FontWeight::EXTRA_BOLD),
        |_: &mut DateTime<Local>| {
            println!("Time button clicked");
        },
    )
    .padding(Padding::all(2.0))
    .background_color(palette::css::TRANSPARENT)
    .active_background_color(palette::css::LIGHT_GRAY.with_alpha(0.4))
    .hovered_border_color(palette::css::GRAY)
}

fn time(time: &mut DateTime<Local>) -> impl WidgetView<DateTime<Local>> + use<> {
    fork(
        time_button(time),
        worker(
            async |proxy: MessageProxy<DateTime<Local>>, _: UnboundedReceiver<()>| {
                loop {
                    tokio::time::sleep(Duration::from_secs(15)).await;

                    proxy
                        .message(Local::now())
                        .expect("Failed to send time update");
                }
            },
            |_: &mut DateTime<Local>, _: UnboundedSender<()>| {},
            |time: &mut DateTime<Local>, new_time: DateTime<Local>| {
                *time = new_time;
            },
        ),
    )
}

fn app_logic(_: &mut AppState) -> impl WidgetView<AppState> + use<> {
    flex_row((
        (FlexSpacer::Fixed(2.px()), label("activites")),
        lens(time, |state: &mut AppState| &mut state.time),
        (label("system"), FlexSpacer::Fixed(2.px())),
    ))
    .cross_axis_alignment(CrossAxisAlignment::Center)
    .main_axis_alignment(MainAxisAlignment::SpaceBetween)
    .background(Background::Color(palette::css::BLACK))
}

fn main() -> Result<(), EventLoopError> {
    let app = Xilem::new_simple(
        AppState { time: Local::now() },
        app_logic,
        WindowOptions::new("bar.top")
            .with_layer_shell()
            .with_anchor(Anchor::TOP)
            .with_initial_inner_size(LogicalSize::new(1920, 24).to_physical::<u32>(1.11111))
            .with_exclusive_zone(24),
    );
    app.run_in(EventLoop::builder())?;
    Ok(())
}
