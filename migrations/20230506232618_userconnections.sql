CREATE TABLE IF NOT EXISTS user_connections (
    id SERIAL PRIMARY KEY,
    user_id BIGINT UNSIGNED,
    connection_date DATE NOT NULL,
    connection_count INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

