-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    age INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

-- Create index on email for faster lookups
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

-- Create index on age for filtering
CREATE INDEX IF NOT EXISTS idx_users_age ON users(age);

-- Insert some test data
INSERT INTO users (email, name, age) VALUES
    ('alice@example.com', 'Alice Smith', 30),
    ('bob@example.com', 'Bob Johnson', 25),
    ('carol@example.com', 'Carol Williams', 35);
