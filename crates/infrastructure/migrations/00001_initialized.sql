CREATE SCHEMA IF NOT EXISTS "user";
CREATE SCHEMA if not exists "permission";
CREATE SCHEMA if not exists "todo";
CREATE table if not exists "permission"."tbl_permission"(
    pk_permission_id SERIAL PRIMARY KEY,
    permission_name VARCHAR(100) UNIQUE NOT NULL
);
create table if not exists "permission"."tbl_role"(
    pk_role_id SERIAL PRIMARY KEY,
    role_name VARCHAR(100) UNIQUE NOT NULL
);
CREATE TABLE IF NOT EXISTS "permission"."tbl_role_permission"(
    fk_role_id INT NOT NULL,
    fk_permission_id INT NOT NULL,
    PRIMARY KEY (fk_role_id, fk_permission_id),
    FOREIGN KEY (fk_role_id) REFERENCES "permission"."tbl_role"(pk_role_id) ON DELETE CASCADE,
    FOREIGN KEY (fk_permission_id) REFERENCES "permission"."tbl_permission"(pk_permission_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS "user"."tbl_user"(
    pk_user_id BIGSERIAL PRIMARY KEY,
    username VARCHAR(150) UNIQUE NOT NULL,
    password_hash VARCHAR(255) DEFAULT NULL
);
CREATE TABLE IF NOT EXISTS "user"."tbl_user_role"(
    fk_user_id BIGINT NOT NULL,
    fk_role_id INT NOT NULL,
    PRIMARY KEY (fk_user_id, fk_role_id),
    FOREIGN KEY (fk_user_id) REFERENCES "user"."tbl_user"(pk_user_id) ON DELETE CASCADE,
    FOREIGN KEY (fk_role_id) REFERENCES "permission"."tbl_role"(pk_role_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "todo"."tbl_todo_item"(
    pk_todo_item_id SERIAL PRIMARY KEY,
    fk_user_id BIGINT NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    is_completed BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (fk_user_id) REFERENCES "user"."tbl_user"(pk_user_id) ON DELETE CASCADE
);
