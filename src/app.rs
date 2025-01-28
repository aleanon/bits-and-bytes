use iced::{widget::{column, container, row, text, text_input, Space}, Element, Length, Padding, Task};

use crate::converter::{Ascii, Binary, Converter, Decimal, Hex};

#[derive(Debug, Clone)]
pub enum Message {
    BinaryInput(String),
    AsciiInput(String),
    HexInput(String),
    DecimalInput(String),
}


#[derive(Debug)]
pub struct Application {
    binary_input: Binary,
    ascii_input: Ascii,
    hex_input: Hex,
    decimal_input: Decimal,
}

impl Application {
    pub fn new() -> (Self, Task<Message>) {
        let app= Application {
            binary_input: Binary::new(),
            ascii_input: Ascii::new(),
            hex_input: Hex::new(),
            decimal_input: Decimal::new(),
        };
        (app, Task::none())
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::BinaryInput(input) => {
                let input = input.chars()
                    .filter(|c| c.is_digit(2) || c.is_ascii_whitespace()).collect();
                self.binary_input.0 = input;
                self.ascii_input = self.binary_input.to_ascii();
                self.hex_input = self.binary_input.to_hex();
                self.decimal_input = self.binary_input.to_decimal();
            }
            Message::AsciiInput(input) => {
                let input = input.chars().filter(|c| c.is_ascii()).collect();
                self.ascii_input.0 = input;
                self.binary_input = self.ascii_input.to_binary();
                self.hex_input = self.ascii_input.to_hex();
                self.decimal_input = self.ascii_input.to_decimal();
            }
            Message::HexInput(input) => {
                let input = input.chars()
                    .filter(|c| c.is_digit(16) || c.is_ascii_whitespace()).collect();
                self.hex_input.0 = input;
                self.binary_input = self.hex_input.to_binary();
                self.ascii_input = self.hex_input.to_ascii();
                self.decimal_input = self.hex_input.to_decimal();
            }
            Message::DecimalInput(input) => {
                let input = input.chars()
                    .filter(|c| c.is_digit(10) || c.is_ascii_whitespace())
                    .collect();
                    
                self.decimal_input.0 = input;
                self.binary_input = self.decimal_input.to_binary();
                self.ascii_input = self.decimal_input.to_ascii();
                self.hex_input = self.decimal_input.to_hex();
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let header = text("Converter")
            .center()
            .width(Length::Fill)
            .size(20);

        let ascii_label = text("ASCII");
        let ascii_input = text_input("Enter Ascii value", &self.ascii_input.0)
            .on_input(Message::AsciiInput)
            .on_paste(Message::AsciiInput);
        let ascii_label_and_input= column![ascii_label, ascii_input].spacing(10);

        let decimal_label = text("Decimal");
        let decimal_input = text_input("Enter Decimal value", &self.decimal_input.0)
            .on_input(Message::DecimalInput)
            .on_paste(Message::DecimalInput);
        let decimal_label_and_input = column![decimal_label, decimal_input].spacing(10);

        let hex_label = text("Hexadecimal");
        let hex_input = text_input("Enter Hex value", &self.hex_input.0)
            .on_input(Message::HexInput)
            .on_paste(Message::HexInput);
        let hex_label_and_input= column![hex_label, hex_input].spacing(10);

        let binary_label = text("Binary");
        let binary_input = text_input("Enter Binary value", &self.binary_input.0)
            .on_input(Message::BinaryInput)
            .on_paste(Message::BinaryInput);
        let binary_label_and_input = column![binary_label, binary_input].spacing(10);

        let content = column![header, ascii_label_and_input, decimal_label_and_input, hex_label_and_input, binary_label_and_input]
            .spacing(40)
            .width(Length::FillPortion(8))
            .padding(Padding{top: 60., ..Default::default()});

        
        let content_with_spacing = row![
            Space::new(Length::FillPortion(1), Length::Fill), 
            content,
            Space::new(Length::FillPortion(1), Length::Fill)
        ];

        container(content_with_spacing)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
               
    }
}