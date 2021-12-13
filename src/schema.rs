table! {
    KanbanBoards (boardID) {
        boardID -> Integer,
        boardName -> Text,
    }
}

table! {
    KanbanColumns (columnID) {
        columnID -> Integer,
        boardID -> Integer,
        columnName -> Text,
    }
}

table! {
    KanbanTasks (taskID) {
        taskID -> Integer,
        columnID -> Integer,
        taskName -> Text,
        taskDescription -> Text,
    }
}

joinable!(KanbanColumns -> KanbanBoards (boardID));
joinable!(KanbanTasks -> KanbanColumns (columnID));

allow_tables_to_appear_in_same_query!(
    KanbanBoards,
    KanbanColumns,
    KanbanTasks,
);
