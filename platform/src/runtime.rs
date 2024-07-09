use iced::Element;

use crate::{glue, Program};

pub fn run(program: Program, settings: glue::Settings) -> iced::Result {
    let (settings, window_settings) = convert::settings(settings);

    iced::application("Roc + Iced <3", App::update, App::view)
        .settings(settings)
        .window(window_settings)
        .run_with(move || App {
            program: program.clone(),
        })
}

#[derive(Debug, Clone)]
struct Message(crate::Message);

unsafe impl Send for Message {}

#[derive(Debug)]
struct App {
    program: crate::Program,
}

impl App {
    fn update(&mut self, message: Message) {
        self.program.update(message.0);
    }

    fn view<'a>(&'a self) -> Element<'a, Message> {
        convert::element(&self.program.view())
    }
}

mod convert {
    use iced::widget::{button, checkbox, column, container, row, text, text_input};

    use super::Message;
    use crate::glue;

    pub fn element(roc_elem: &glue::Element) -> iced::Element<'static, Message> {
        match roc_elem.tag() {
            glue::ElementTag::Button => {
                let inner = roc_elem.button();

                let style_fn = inner.style.clone();

                let mut button = button(element(&inner.content))
                    .padding(padding(inner.padding))
                    .clip(inner.clip)
                    .style(move |theme, status| {
                        style_fn
                            .force_thunk(button_status(status))
                            .into_option()
                            .map(button_style)
                            .unwrap_or_else(|| button::primary(theme, status))
                    });

                if let Some(action) = inner.on_press.active().cloned() {
                    button = button.on_press(Message(action));
                }

                if let Some(width) = inner.width.as_option().copied().map(length) {
                    button = button.width(width);
                }

                if let Some(height) = inner.height.as_option().copied().map(length) {
                    button = button.height(height);
                }

                button.into()
            }
            glue::ElementTag::Checkbox => {
                let inner = roc_elem.checkbox();

                let mut checkbox = checkbox(inner.label.as_str(), inner.is_checked);

                if let Some(action) = inner.on_toggle.active().cloned() {
                    checkbox = checkbox
                        .on_toggle(move |is_checked| Message(action.force_thunk(is_checked)));
                }

                checkbox.into()
            }
            glue::ElementTag::Column => {
                let inner = roc_elem.column();

                column(inner.children.iter().map(element))
                    .width(length(inner.width))
                    .height(length(inner.height))
                    .max_width(inner.max_width)
                    .padding(padding(inner.padding))
                    .spacing(inner.spacing)
                    .align_items(alignment(inner.align_items))
                    .clip(inner.clip)
                    .into()
            }
            glue::ElementTag::Container => {
                let inner = roc_elem.container();

                let style = container::Style {
                    text_color: color_opt(&inner.style.text_color),
                    background: color_opt(&inner.style.background).map(Into::into),
                    border: border(inner.style.border),
                    // TODO
                    shadow: Default::default(),
                };

                let mut container = container(element(&inner.content))
                    .padding(padding(inner.padding))
                    .max_width(inner.max_width)
                    .max_height(inner.max_height)
                    .align_x(horizontal_alignment(inner.horizontal_alignment))
                    .align_y(vertical_alignment(inner.vertical_alignment))
                    .clip(inner.clip)
                    .style(move |_| style);

                if let Some(width) = inner.width.as_option().copied().map(length) {
                    container = container.width(width);
                }

                if let Some(height) = inner.height.as_option().copied().map(length) {
                    container = container.height(height);
                }

                container.into()
            }
            glue::ElementTag::Row => {
                let inner = roc_elem.row();

                row(inner.children.iter().map(element))
                    .width(length(inner.width))
                    .height(length(inner.height))
                    .padding(padding(inner.padding))
                    .spacing(inner.spacing)
                    .align_items(alignment(inner.align_items))
                    .clip(inner.clip)
                    .into()
            }
            glue::ElementTag::Text => {
                let inner = roc_elem.text();

                text(inner.value.to_string()).into()
            }
            glue::ElementTag::TextInput => {
                let inner = roc_elem.text_input();

                let mut text_input =
                    text_input("", inner.value.as_str()).width(length(inner.width));

                if let Some(action) = inner.on_submit.active().cloned() {
                    text_input = text_input.on_submit(Message(action));
                }

                if let Some(action) = inner.on_input.active().cloned() {
                    text_input =
                        text_input.on_input(move |input| Message(action.force_thunk(&input)));
                }

                text_input.into()
            }
        }
    }

    fn length(roc_length: glue::Length) -> iced::Length {
        match roc_length.tag {
            glue::LengthTag::Fill => iced::Length::Fill,
            glue::LengthTag::FillPortion => iced::Length::FillPortion(roc_length.fill_portion()),
            glue::LengthTag::Fixed => iced::Length::Fixed(roc_length.fixed()),
            glue::LengthTag::Shrink => iced::Length::Shrink,
        }
    }

    fn color_opt(c: &glue::Optional<glue::Color>) -> Option<iced::Color> {
        c.as_option().copied().map(color)
    }

    fn color(color: glue::Color) -> iced::Color {
        let glue::Color { r, g, b, a } = color;

        iced::Color { r, g, b, a }
    }

    fn border(b: glue::Border) -> iced::Border {
        iced::Border {
            color: color(b.color),
            width: b.width,
            radius: b.radius.into(),
        }
    }

    fn padding(p: glue::Padding) -> iced::Padding {
        let glue::Padding {
            bottom,
            left,
            right,
            top,
        } = p;

        iced::Padding {
            top,
            right,
            bottom,
            left,
        }
    }

    fn alignment(a: glue::Alignment) -> iced::Alignment {
        match a {
            glue::Alignment::Start => iced::Alignment::Start,
            glue::Alignment::Center => iced::Alignment::Center,
            glue::Alignment::End => iced::Alignment::End,
        }
    }

    fn horizontal_alignment(a: glue::HorizontalAlignment) -> iced::alignment::Horizontal {
        match a {
            glue::HorizontalAlignment::Center => iced::alignment::Horizontal::Center,
            glue::HorizontalAlignment::Left => iced::alignment::Horizontal::Left,
            glue::HorizontalAlignment::Right => iced::alignment::Horizontal::Right,
        }
    }

    fn vertical_alignment(a: glue::VerticalAlignment) -> iced::alignment::Vertical {
        match a {
            glue::VerticalAlignment::Bottom => iced::alignment::Vertical::Bottom,
            glue::VerticalAlignment::Center => iced::alignment::Vertical::Center,
            glue::VerticalAlignment::Top => iced::alignment::Vertical::Top,
        }
    }

    fn button_status(s: iced::widget::button::Status) -> glue::ButtonStatus {
        match s {
            iced::widget::button::Status::Active => glue::ButtonStatus::Active,
            iced::widget::button::Status::Disabled => glue::ButtonStatus::Disabled,
            iced::widget::button::Status::Hovered => glue::ButtonStatus::Hovered,
            iced::widget::button::Status::Pressed => glue::ButtonStatus::Pressed,
        }
    }

    fn button_style(s: glue::ButtonStyle) -> button::Style {
        button::Style {
            background: s.background.as_option().copied().map(color).map(From::from),
            text_color: color(s.text_color),
            border: border(s.border),
            shadow: iced::Shadow::default(),
        }
    }

    pub fn settings(s: glue::Settings) -> (iced::Settings, iced::window::Settings) {
        let glue::Settings {
            window,
            antialiasing,
            default_text_size,
        } = s;

        let settings = iced::Settings {
            antialiasing,
            default_text_size: default_text_size.into(),
            ..Default::default()
        };

        let glue::WindowSettings {
            max_size,
            min_size,
            size,
            decorations,
            resizable,
            transparent,
        } = window;

        let window_settings = iced::window::Settings {
            size: iced::Size::new(size.width, size.height),
            min_size: min_size
                .as_option()
                .map(|size| iced::Size::new(size.width, size.height)),
            max_size: max_size
                .as_option()
                .map(|size| iced::Size::new(size.width, size.height)),
            decorations,
            resizable,
            transparent,
            ..Default::default()
        };

        (settings, window_settings)
    }
}
