use crate::view::View;

use chrono_tz::TZ_VARIANTS;
use eframe::egui::{self, Ui};

pub struct TimezoneDropdown {
    selected: chrono_tz::Tz,
    id: u64,
    id_string: String,
}

impl TimezoneDropdown {
    pub fn new(id: u64) -> Self {
        Self {
            selected: chrono_tz::Tz::UTC,
            id,
            id_string: format!("timezone_dropdown_{}", id),
        }
    }

    pub fn change_id(&mut self, id: u64) {
        self.id = id;
        self.id_string = format!("timezone_dropdown_{}", id);
    }
}

impl View for TimezoneDropdown {
    fn ui(&mut self, ui: &mut Ui) {
        egui::ComboBox::from_id_source(&self.id_string)
            .selected_text(self.selected.name())
            .show_ui(ui, |ui| {
                for timezone in TZ_VARIANTS.iter() {
                    ui.selectable_value(&mut self.selected, timezone.clone(), timezone.name());
                }
            });
    }
}

impl TimezoneDropdown {
    pub fn selected(&self) -> chrono_tz::Tz {
        self.selected
    }
}
