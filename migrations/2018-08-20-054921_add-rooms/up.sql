-- Your SQL goes here

create table rooms (
	id integer primary key not null,
	room_name text not null,
	created_at timestamp default current_timestamp not null,
	updated_at timestamp default current_timestamp not null,
	check (room_name <> '')
);

create trigger rooms after update on rooms
begin
	update rooms set updated_at = current_timestamp where id = NEW.id;
end;

insert into rooms (room_name) values ("Default Room");

alter table devices add room_id integer not null references rooms(id) on delete cascade on update cascade default 1;
