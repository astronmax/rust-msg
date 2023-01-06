drop database if exists rustMsg;
create database rustMsg;
use rustMsg;

drop table if exists users;
create table users(
	user_id int primary key auto_increment not null,
	name varchar(45),
	password varchar(32)
);

drop table if exists direct_msgs;
create table direct_msgs(
	msg_id int primary key auto_increment not null,
	body text not null,
	
	from_id int not null,
	foreign key (from_id) references users(user_id),

	to_id int not null,
	foreign key (to_id) references users(user_id),

	dt datetime
);

drop procedure if exists get_direct_chats;
delimiter $$
create procedure get_direct_chats(in user_name varchar(45))
begin
	declare needed_id int;
	select user_id from users where name = user_name into needed_id;
	select name from (
		select from_id from direct_msgs where to_id = needed_id group by from_id
		union
		select to_id from direct_msgs where from_id = needed_id group by to_id
	) t inner join users on users.user_id = t.from_id;
end$$
delimiter ;

drop procedure if exists add_direct_msg;
delimiter $$
create procedure add_direct_msg(in body text, in src varchar(45), in dst varchar(45), in msg_dt datetime)
begin
	declare src_id int;
	declare dst_id int;

	select user_id from users where name = src into src_id;
	select user_id from users where name = dst into dst_id;
	insert into direct_msgs(body, from_id, to_id, dt) values (body, src_id, dst_id, msg_dt);
end$$
delimiter ;

drop procedure if exists get_direct_msgs;
delimiter $$
create procedure get_direct_msgs(in from_user varchar(45), in to_user varchar(45))
begin
	declare from_user_id int;
	declare to_user_id int;
	select user_id from users where name = from_user into from_user_id;
	select user_id from users where name = to_user into to_user_id;

	select body, name, dt from direct_msgs T1, users T2 where T1.to_id = from_user_id and T1.from_id = T2.user_id
	union
	select body, name, dt from direct_msgs T1, users T2 where T1.to_id = to_user_id and T1.from_id = T2.user_id
	order by dt;
end$$
delimiter ;