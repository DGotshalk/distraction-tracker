CREATE TABLE If NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    ip_address VARCHAR(45) NOT NULL,
	user_agent VARCHAR(255) NOT NULL,
	UNIQUE KEY (ip_address, user_agent)
);
