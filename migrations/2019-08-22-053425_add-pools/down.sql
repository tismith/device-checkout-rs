-- This file should undo anything in `up.sql`
create temporary table devices_backup(id, device_name, device_url, device_owner, comments, reservation_status, created_at, updated_at);

insert into devices_backup select id, device_name, device_url, device_owner, comments, reservation_status, created_at, updated_at from devices;

drop table devices;

--copied from the previous migration
create table devices (
	id integer primary key not null,
	device_name text unique not null,
	device_url text,
	device_owner text,
	comments text,
	reservation_status text not null default 'available',
	created_at timestamp default current_timestamp not null,
	updated_at timestamp default current_timestamp not null,
	--device_name not empty
	check (device_name <> '')
	--reservation_status is an enum
	check(reservation_status in ('available', 'reserved'))
	--if we're reserved, then we need a not empty device_owner
	check (reservation_status <> "reserved" or (device_owner is not null and device_owner <> ''))
);

insert into devices select id, device_name, device_url, device_owner, comments, reservation_status, created_at, updated_at from devices_backup;

drop table devices_backup;

create trigger devices after update on devices
begin
	update devices set updated_at = current_timestamp where id = NEW.id;
end;

drop table pools;
