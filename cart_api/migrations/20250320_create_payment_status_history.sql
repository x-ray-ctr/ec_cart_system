CREATE TABLE payment_status_history (
    id SERIAL PRIMARY KEY,
    payment_id INT NOT NULL REFERENCES payments(id) ON DELETE CASCADE,
    status TEXT NOT NULL,
    changed_at TIMESTAMP NOT NULL DEFAULT NOW()
);
