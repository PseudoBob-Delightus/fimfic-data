CREATE TABLE IF NOT EXISTS Loops (
    id           integer PRIMARY KEY autoincrement,
    start_time   integer NOT NULL,
    end_time     integer NOT NULL,
    version      integer NOT NULL
);