-- Your SQL goes here
create table devices (
	device_name text unique not null,
	url text,
	device_owner text,
	comments text,
	reservation_status text
);
