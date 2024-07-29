package db

import "database/sql"

func CreateTableUsers(db *sql.DB) error {
	table := `
 CREATE TABLE IF NOT EXISTS Users (
        id INTEGER PRIMARY KEY,
		lastname TEXT NOT NULL,
		firstname TEXT NOT NULL	
	);
    `
	_, err := db.Exec(table)
	return err
}

func CreateTableEntreprise(db *sql.DB) error {
	table := `
	CREATE TABLE IF NOT EXISTS Entreprises(
		id INTEGER PRIMARY KEY,
		user_id INTEGER,
		name TEXT NOT NULL,
		date TEXT NOT NULL,
		codeape TEXT NOT NULL,
		status TEXT NOT NULL,
		jrsttx INTEGER,
		jrsweek INTEGER,
		jrsferies INTEGER,
		jrscp INTEGER,
		jan INTEGER,
		fev INTEGER,
		mar INTEGER,
		avr INTEGER,
		mai INTEGER,
		juin INTEGER,
		jui INTEGER,
		aout INTEGER,
		sept INTEGER,
		oct INTEGER,
		nov INTEGER,
		dec INTEGER,
		FOREIGN KEY(user_id) REFERENCES Users(id)
	)`
	_, err := db.Exec(table)
	return err
}

// Since totalservice, is autocompletion on myform
func CreateTableActivite(db *sql.DB) error {
	table := `
	CREATE TABLE IF NOT EXISTS Activites(
		id INTEGER PRIMARY KEY,
		user_id INTEGER,
		production INTEGER,
		entretien INTEGER,
		clientele INTEGER,
		interprofession INTEGER,
		formation INTEGER,
		prodjour INTEGER,
		totalservice INTEGER,
		tva REAL,
		moyprix REAL,
		donttva REAL,
		totalmoyprix REAL,
		htjours REAL,
		htcanann REAL,
		tvaann REAL,
		ttcann REAL,
		FOREIGN KEY(user_id) REFERENCES Users(id)
	)`
	_, err := db.Exec(table)
	return err
}
