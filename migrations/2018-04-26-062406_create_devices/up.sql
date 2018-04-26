-- Your SQL goes here
create table devices (
	id integer primary key not null,
	device_name text unique not null,
	device_url text,
	device_owner text,
	comments text,
	reservation_status text
);
