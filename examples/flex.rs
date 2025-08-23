// Copyright 2024 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

//! Flex properties can be set in Xilem.

use winit::dpi::LogicalSize;
use winit::error::EventLoopError;
use winit::platform::wayland::Anchor;
use xilem::masonry::properties::types::AsUnit;
use xilem::style::{Background, Style};
use xilem::view::{
    CrossAxisAlignment, FlexExt as _, FlexSpacer, Label, MainAxisAlignment, button, flex_row,
    label, sized_box,
};
use xilem::{EventLoop, WidgetView, WindowOptions, Xilem, palette};

/// A component to make a bigger than usual button
fn big_button(
    label: impl Into<Label>,
    callback: impl Fn(&mut i32) + Send + Sync + 'static,
) -> impl WidgetView<i32> {
    sized_box(button(label, callback))
        .width(40.px())
        .height(40.px())
}

fn app_logic(data: &mut i32) -> impl WidgetView<i32> + use<> {
    flex_row((
        FlexSpacer::Fixed(30.px()),
        big_button("-", |data| {
            *data -= 1;
        }),
        FlexSpacer::Flex(1.0),
        label(format!("count: {data}")).text_size(32.).flex(5.0),
        FlexSpacer::Flex(1.0),
        big_button("+", |data| {
            *data += 1;
        }),
        FlexSpacer::Fixed(30.px()),
    ))
    .cross_axis_alignment(CrossAxisAlignment::Center)
    .main_axis_alignment(MainAxisAlignment::Center)
    .background(Background::Color(palette::css::ALICE_BLUE))
}

fn main() -> Result<(), EventLoopError> {
    let app = Xilem::new_simple(
        0,
        app_logic,
        WindowOptions::new("Centered Flex")
            .with_layer_shell()
            .with_anchor(Anchor::TOP)
            .with_initial_inner_size(LogicalSize::new(1920, 360).to_physical::<u32>(1.11111)),
    );
    app.run_in(EventLoop::builder())?;
    Ok(())
}
