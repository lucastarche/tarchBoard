use super::models::{KanbanBoard, KanbanColumn, KanbanTask, NewKanbanBoard, NewKanbanColumn};
use crate::schema::{KanbanBoards, KanbanColumns, KanbanTasks};
use crate::utility_widgets::TextInput;
use crate::view::{UiWidget, View};

use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};
use eframe::egui::{menu, Ui, Window};
use std::mem;
use std::rc::Rc;

pub struct KanbanWidget {
    db: Rc<SqliteConnection>,

    current_board: Option<KanbanBoard>,
    new_board: TextInput,
    new_column: TextInput,
}

impl KanbanWidget {
    pub fn new(db: Rc<SqliteConnection>) -> Self {
        Self {
            db,
            current_board: Default::default(),
            new_board: TextInput::new("Board name".to_string()),
            new_column: TextInput::new("Column name".to_string()),
        }
    }

    fn display_board(&mut self, ui: &mut Ui) {
        if self.current_board.is_none() {
            return;
        }

        // UNWRAP SAFETY: We know self.current_board is not `None`
        let board = self.current_board.clone().unwrap();

        // TODO: Display current board
        let columns = KanbanColumns::table
            .filter(KanbanColumns::boardID.eq(board.board_id))
            .load::<KanbanColumn>(&*self.db)
            .expect("Error loading columns");

        ui.vertical_centered(|ui| ui.heading(&board.board_name));
        ui.horizontal(|ui| {
            for column in &columns {
                let tasks = KanbanTasks::table
                    .filter(KanbanTasks::columnID.eq(column.column_id))
                    .load::<KanbanTask>(&*self.db)
                    .expect("Error loading tasks");

                ui.vertical(|ui| {
                    ui.heading(&column.column_name);
                    for task in &tasks {
                        ui.label(&task.task_name);
                    }
                });
            }

            self.new_column.ui(ui);
        });

        if self.new_column.clicked() {
            let new_column = NewKanbanColumn {
                column_name: mem::replace(&mut self.new_column.value, String::new()),
                board_id: board.board_id,
            };

            insert_into(KanbanColumns::table)
                .values(&new_column)
                .execute(&*self.db)
                .expect("Error inserting new KanbanColumn");
        }
    }
}

impl UiWidget for KanbanWidget {
    fn name(&self) -> &'static str {
        "Kanban"
    }

    fn show(&mut self, ctx: &eframe::egui::CtxRef) {
        Window::new(self.name())
            .resizable(true)
            .show(ctx, |ui| self.ui(ui));
    }
}

impl View for KanbanWidget {
    fn ui(&mut self, ui: &mut Ui) {
        menu::bar(ui, |ui| {
            menu::menu(ui, "Boards", |ui| {
                let boards: Vec<KanbanBoard> = KanbanBoards::table
                    .load::<KanbanBoard>(&*self.db)
                    .expect("Error loading boards");

                for board in boards {
                    if ui.button(&board.board_name).clicked() {
                        self.current_board = Some(board.clone());
                    }
                }
            });
        });

        self.display_board(ui);

        ui.separator();

        ui.heading("Create new board");
        self.new_board.ui(ui);
        if self.new_board.clicked() {
            let new_board = NewKanbanBoard {
                board_name: mem::replace(&mut self.new_board.value, String::new()),
            };

            insert_into(KanbanBoards::table)
                .values(&new_board)
                .execute(&*self.db)
                .expect("Error inserting new KanbanBoard");
        }
    }
}
