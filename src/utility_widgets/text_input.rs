use crate::view::View;

use eframe::egui::{TextEdit, Ui};

pub struct TextInput {
    pub value: String,
    hint_text: String,
    width: f32,
    clicked: bool,
}

impl TextInput {
    pub fn new(hint_text: String) -> Self {
        Self {
            value: String::new(),
            clicked: false,
            width: 75.0,
            hint_text,
        }
    }

    pub fn clicked(&self) -> bool {
        self.clicked
    }
}

impl View for TextInput {
    fn ui(&mut self, ui: &mut Ui) {
        self.clicked = false;
        ui.horizontal(|ui| {
            let text_edit = TextEdit::singleline(&mut self.value)
                .hint_text(&self.hint_text)
                .desired_width(self.width);
            ui.add(text_edit);
            if ui.small_button("+").clicked() {
                self.clicked = true;
            }
        });
    }
}
