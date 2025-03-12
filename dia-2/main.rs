use iced::{
    Alignment::Center,
    Border, Color, Element, Font, font,
    widget::{button, column, row, text, text_input},
};

#[derive(Debug, Clone)]
enum Message {
    TextChanged(String),
    Add,
    Clear,
    ClearFinished,
    ToggleCheck(usize),
    Remove(usize),
}

#[derive(Default)]
struct Todolist {
    text_input: String,
    list: Vec<(String, bool)>,
}

impl Todolist {
    pub fn view(&self) -> Element<'_, Message> {
        column![
            text("TODOLIST"),
            row![
                text_input("Tarea", &self.text_input)
                    .on_input(Message::TextChanged)
                    .on_submit(Message::Add)
                    .padding(10),
                button("Agregar").on_press(Message::Add)
            ]
            .spacing(10)
            .padding(10)
            .align_y(Center),
            row![
                text("Tareas").size(20).font(Font {
                    weight: font::Weight::Bold,
                    ..Default::default()
                }),
                button("Limpiar todas").on_press(Message::Clear),
                button("Limpiar completadas").on_press(Message::ClearFinished),
            ]
            .spacing(20)
            .align_y(Center),
            column![self.list.iter().enumerate().fold(
                column![].padding(10).spacing(10).align_x(Center),
                |column, (index, (task, done))| {
                    let text = if *done {
                        text(task)
                            .font(Font {
                                style: font::Style::Italic,
                                ..Default::default()
                            })
                            .color(Color::from_rgba(0., 0., 0., 0.5))
                    } else {
                        text(task)
                    };

                    let row = row![
                        if *done {
                            button("")
                                .on_press(Message::ToggleCheck(index))
                                .width(20)
                                .height(20)
                                .style(|_, _| button::Style {
                                    background: Some(iced::Background::Color(Color::from_rgb(
                                        0.1, 0.3, 0.72,
                                    ))),
                                    ..Default::default()
                                })
                        } else {
                            button("")
                                .on_press(Message::ToggleCheck(index))
                                .width(20)
                                .height(20)
                                .style(|_, _| button::Style {
                                    background: Some(iced::Background::Color(Color::WHITE)),
                                    border: Border {
                                        width: 1.0,
                                        color: Color::BLACK,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                        },
                        text,
                        button("Eliminar").on_press(Message::Remove(index))
                    ]
                    .spacing(10)
                    .align_y(Center);

                    column.push(row)
                }
            )]
            .align_x(Center)
            .spacing(10),
        ]
        .align_x(Center)
        .spacing(20)
        .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::TextChanged(content) => {
                self.text_input = content;
            }
            Message::Add => {
                if self.text_input.is_empty() {
                    println!("Campo vacío");
                    return;
                }

                self.list.push((self.text_input.trim().to_string(), false));

                self.text_input.clear();
                println!("{:?}", self.list);
            }
            Message::Clear => {
                self.list.clear();
            }
            Message::ClearFinished => {
                self.list.retain(|(_, done)| !*done);
            }
            Message::Remove(index) => {
                self.list.remove(index);
            }
            Message::ToggleCheck(index) => {
                if let Some((_task, done)) = self.list.get_mut(index) {
                    *done = !*done;
                }
            }
        }
    }
}

fn main() -> iced::Result {
    iced::run("TodoList", Todolist::update, Todolist::view)
}
