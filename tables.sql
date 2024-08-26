CREATE TABLE user_data (
    id TEXT PRIMARY KEY,
    gender TEXT,
    age INTEGER,
    salary INTEGER,
    purchased INTEGER
);

INSERT INTO user_data (id, gender, age, salary, purchased) VALUES ($1, $2, $3, $4, $5)

DROP TABLE user_data;