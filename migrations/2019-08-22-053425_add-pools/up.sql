-- Your SQL goes here

create table pools (
	id integer primary key not null,
	pool_name text not null,
	created_at timestamp default current_timestamp not null,
	updated_at timestamp default current_timestamp not null,
	check (pool_name <> '')
);

create trigger pools after update on pools
begin
	update pools set updated_at = current_timestamp where id = NEW.id;
end;

insert into pools (pool_name) values ('Default Pool');

create temporary table devices_backup(id, device_name, device_url, device_owner, comments, reservation_status, created_at, updated_at);

insert into devices_backup select id, device_name, device_url, device_owner, comments, reservation_status, created_at, updated_at from devices;

drop table devices;

create table devices (
	id integer primary key not null,
	device_name text unique not null,
	pool_id integer not null references pools(id),
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

insert into devices (id, device_name, device_url, device_owner, comments, reservation_status, created_at, updated_at, pool_id)
select devices_backup.id, device_name, device_url, device_owner, comments, reservation_status, devices_backup.created_at, devices_backup.updated_at, pools.id
from devices_backup, pools
where pools.pool_name = 'Default Pool' ;

drop table devices_backup;

create trigger devices after update on devices
begin
	update devices set updated_at = current_timestamp where id = NEW.id;
end;
