CREATE TABLE IF NOT EXISTS logs (
  id SERIAL PRIMARY KEY,
  message TEXT,
  target VARCHAR(255),
  level VARCHAR(10),
  logged_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_logs_level ON logs(level); -- helps with WHERE level = "info" etc
CREATE INDEX idx_logs_logged_at ON logs(logged_at DESC); -- helps with WHERE logged_at > or between
CREATE INDEX idx_logs_level_logged_at ON logs(level, logged_at DESC); -- helps with WHERE level and BETWEEN / WHERE logged at ..

