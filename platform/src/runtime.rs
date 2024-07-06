use iced::Element;

use crate::Program;

pub fn run(program: Program) -> iced::Result {
    iced::application("Roc + Iced <3", App::update, App::view).run_with(move || App {
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

                let mut button = button(element(&inner.content));

                if let Some(action) = inner.on_press.active().cloned() {
                    button = button.on_press(Message(action));
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

                let children = &inner.children;

                column(children.iter().map(element)).into()
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
                    .style(move |_| style);

                if let Some(width) = length(inner.width) {
                    if inner.center_x {
                        container = container.center_x(width);
                    } else {
                        container = container.width(width);
                    }
                }

                if let Some(height) = length(inner.height) {
                    if inner.center_y {
                        container = container.center_y(height);
                    } else {
                        container = container.height(height);
                    }
                }

                container.into()
            }
            glue::ElementTag::Row => {
                let inner = roc_elem.row();

                row(inner.children.iter().map(element)).into()
            }
            glue::ElementTag::Text => {
                let inner = roc_elem.text();

                text(inner.value.to_string()).into()
            }
            glue::ElementTag::TextInput => {
                let inner = roc_elem.text_input();

                let mut text_input = text_input("", inner.value.as_str());

                if let Some(width) = length(inner.width) {
                    text_input = text_input.width(width);
                }

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

    fn length(roc_length: glue::Length) -> Option<iced::Length> {
        Some(match roc_length.tag {
            glue::LengthTag::Fill => iced::Length::Fill,
            glue::LengthTag::FillPortion => iced::Length::FillPortion(roc_length.fill_portion()),
            glue::LengthTag::Fixed => iced::Length::Fixed(roc_length.fixed()),
            glue::LengthTag::Shrink => iced::Length::Shrink,
            glue::LengthTag::Unspecified => return None,
        })
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
}
