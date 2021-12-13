use crate::schema::{KanbanBoards, KanbanColumns};
use diesel::Queryable;

#[derive(Queryable, Clone)]
pub struct KanbanBoard {
    pub board_id: i32,
    pub board_name: String,
}

#[derive(Queryable, Clone)]
pub struct KanbanColumn {
    pub column_id: i32,
    pub board_id: i32,
    pub column_name: String,
}

#[derive(Queryable, Clone)]
pub struct KanbanTask {
    pub task_id: i32,
    pub column_id: i32,
    pub task_name: String,
    pub task_description: String,
}

#[derive(Insertable)]
#[table_name = "KanbanBoards"]
pub struct NewKanbanBoard {
    #[column_name = "boardName"]
    pub board_name: String,
}

#[derive(Insertable)]
#[table_name = "KanbanColumns"]
pub struct NewKanbanColumn {
    #[column_name = "columnName"]
    pub column_name: String,

    #[column_name = "boardID"]
    pub board_id: i32,
}
