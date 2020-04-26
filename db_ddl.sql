CREATE TABLE apps (id int primary key, name varchar(50) not null, version VARCHAR(50) not null);
CREATE TABLE db_versions (id int primary key, version VARCHAR(50) not null);
CREATE TABLE apps_db_versions (app_id int, db_id int, PRIMARY KEY(app_id, db_id), FOREIGN KEY(app_id) REFERENCES apps(id), FOREIGN KEY(db_id) REFERENCES db_versions(id));

INSERT INTO apps(id, name , version) VALUES (1, 'app_name', '1.0.0');
INSERT INTO db_versions(id, version) VALUES (1, 'v1');
INSERT INTO apps_db_versions (app_id, db_id) VALUES (1, 1);
