-- Your SQL goes here
create table devices (
	id integer primary key not null,
	device_name text unique not null,
	device_url text,
	device_owner text check((reservation_status = 'reserved' and device_owner is not null) or (reservation_status = 'available')),
	comments text,
	reservation_status text check(reservation_status in ('available', 'reserved')) not null default 'available',
	created_at timestamp default current_timestamp not null,
	updated_at timestamp default current_timestamp not null
);

create trigger devices after update on devices
begin
	update devices set updated_at = datetime('now') where id = NEW.id;
end;

insert into devices (device_name, device_url) values ("unit1", "http://unit1");
insert into devices (device_name, device_url) values ("unit2", "http://unit2");
