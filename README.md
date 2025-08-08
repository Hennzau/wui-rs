# wui-rs

WUI (Wayland User Interface) is a rust framework to write wayland native GUI applications for smithay-based compositor such as bars, menus, notifications daemon etc...

It is based on a MVC (Model View Controller) architecture, similar to Elm and Iced. Which means that applications written with WUI split the logic into 3 states: the model, which is the data of the application ; the view, which is how the data should be rendered to the screen ; and the controller, which updates the data based on user interactions from the view.
