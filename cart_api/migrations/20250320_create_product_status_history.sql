CREATE TABLE product_status_history (
    id SERIAL PRIMARY KEY,
    product_id INT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    status TEXT NOT NULL,
    changed_at TIMESTAMP NOT NULL DEFAULT NOW()
)