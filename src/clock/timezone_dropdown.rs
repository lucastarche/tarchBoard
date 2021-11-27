use crate::view::View;

use chrono_tz::TZ_VARIANTS;
use eframe::egui::{self, Ui};

pub struct TimezoneDropdown {
    selected: chrono_tz::Tz,
}

impl Default for TimezoneDropdown {
    fn default() -> Self {
        Self {
            selected: chrono_tz::Tz::UTC,
        }
    }
}

impl View for TimezoneDropdown {
    fn ui(&mut self, ui: &mut Ui) {
        egui::ComboBox::from_label("Select Timezone")
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
