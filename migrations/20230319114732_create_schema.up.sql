CREATE TABLE rooms (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
	name VARCHAR(50) NOT NULL UNIQUE,
	description TEXT NOT NULL
);

CREATE TABLE devices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    room INTEGER,
    parent INTEGER,
    type VARCHAR(50) NOT NULL,
    is_on INTEGER NOT NULL,

    FOREIGN KEY (room) 
        REFERENCES rooms (id)
            ON DELETE CASCADE 
            ON UPDATE NO ACTION,
    
    FOREIGN KEY (parent) 
        REFERENCES devices (id)
);
