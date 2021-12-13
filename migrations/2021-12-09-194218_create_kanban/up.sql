-- Your SQL goes here
--
CREATE TABLE "KanbanBoards" (
	"boardID"	INTEGER UNIQUE NOT NULL,
	"boardName"	TEXT NOT NULL,
	PRIMARY KEY("boardID" AUTOINCREMENT)
);

CREATE TABLE "KanbanColumns" (
	"columnID"	INTEGER UNIQUE NOT NULL,
	"boardID"	INTEGER NOT NULL,
	"columnName"	TEXT NOT NULL,
	FOREIGN KEY("boardID") REFERENCES "KanbanBoards"("boardID"),
	PRIMARY KEY("columnID" AUTOINCREMENT)
);

CREATE TABLE "KanbanTasks" (
	"taskID"	INTEGER NOT NULL,
	"columnID"	INTEGER NOT NULL,
	"taskName"	TEXT NOT NULL,
	"taskDescription"	TEXT NOT NULL,
    FOREIGN KEY("columnID") REFERENCES "KanbanColumns"("columnID"),
	PRIMARY KEY("taskID" AUTOINCREMENT)
);
