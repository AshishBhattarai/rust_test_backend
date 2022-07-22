-- Your SQL goes here

CREATE TABLE users (
	-- fields
	id uuid NOT NULL DEFAULT uuid_generate_v4(),
	first_name varchar(50) NOT NULL,
	last_name varchar(50) NOT NULL,
	email varchar(50) NOT NULL,
	username varchar(50) NOT NULL,
	password varchar(255) NOT NULL,
	-- constraints
	PRIMARY KEY(id),
	UNIQUE(email, username)
);
