CREATE DATABASE IF NOT EXISTS sql_forge_test;
USE sql_forge_test;

CREATE TABLE IF NOT EXISTS users (
    id      BIGINT AUTO_INCREMENT PRIMARY KEY,
    name    VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS organisations (
    id      BIGINT AUTO_INCREMENT PRIMARY KEY,
    name    VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS categories (
    id      BIGINT AUTO_INCREMENT PRIMARY KEY,
    name    VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS items (
    id          BIGINT AUTO_INCREMENT PRIMARY KEY,
    name        VARCHAR(255) NOT NULL,
    category_id BIGINT       NOT NULL,
    price       DECIMAL(12,2) NOT NULL DEFAULT 0,
    stock       INT          NOT NULL DEFAULT 0,
    created_at  DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (category_id) REFERENCES categories(id)
);

CREATE TABLE IF NOT EXISTS products (
    id       BIGINT AUTO_INCREMENT PRIMARY KEY,
    name     VARCHAR(255) NOT NULL,
    price    DECIMAL(12,2) NOT NULL DEFAULT 0,
    stock    INT          NOT NULL DEFAULT 0,
    category VARCHAR(255) NOT NULL DEFAULT ''
);

INSERT IGNORE INTO users (id, name) VALUES
    (1, 'Alice'),
    (2, 'Bob'),
    (3, 'Charlie'),
    (4, 'Diana'),
    (5, 'Eve');

INSERT IGNORE INTO organisations (id, name) VALUES
    (1, 'Org Alpha'),
    (2, 'Org Beta'),
    (3, 'Org Gamma');

INSERT IGNORE INTO categories (id, name) VALUES
    (1, 'Electronics'),
    (2, 'Books'),
    (3, 'Clothing');

INSERT IGNORE INTO items (id, name, category_id, price, stock, created_at) VALUES
    (1,  'Laptop',      1, 1500.00, 10, '2025-01-15 10:00:00'),
    (2,  'Mouse',       1,   25.00, 200, '2025-02-10 12:00:00'),
    (3,  'Keyboard',    1,   80.00, 150, '2025-03-05 09:30:00'),
    (4,  'Rust Book',   2,   45.00, 300, '2025-04-20 14:00:00'),
    (5,  'T-Shirt',     3,   20.00, 500, '2025-05-01 08:00:00'),
    (6,  'Headphones',  1,  120.00, 75,  '2025-05-10 16:00:00'),
    (7,  'Monitor',     1,  350.00, 30,  '2025-06-01 11:00:00');

INSERT IGNORE INTO products (id, name, price, stock, category) VALUES
    (1, 'Smartphone',   400.00,  50, 'Electronics'),
    (2, 'Tablet',       800.00,  30, 'Electronics'),
    (3, 'Notebook',       5.00, 500, 'Stationery'),
    (4, 'Desk Lamp',    35.00, 100, 'Furniture'),
    (5, 'Backpack',     60.00,  80, 'Accessories'),
    (6, 'USB Cable',    10.00,  0,  'Electronics'),
    (7, 'Hard Drive',  120.00,  20, 'Electronics');
