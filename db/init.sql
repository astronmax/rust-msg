create database if not exists rustmsg_db character set utf8 collate utf8_general_ci;
use rustmsg_db;

-- ==============
-- Users table
-- ==============
drop table if exists `users`;
create table `users` (
    `user_id` int primary key auto_increment not null,
    `name` varchar(45),
    `password` varchar(32)
);

-- ==============
-- Chats table
-- ==============
drop table if exists `chats`
create table `chats` (
	`chat_id` int primary key auto_increment not null,
	`user_id` int not null,
	`peer_id` int not null,

	foreign key (`user_id`) references `chats`(`chat_id`),
	foreign key (`peer_id`) references `users`(`user_id`)
);

-- ==============
-- Messages table
-- ==============
drop table if exists `messages`;
create table `messages` (
	`message_id` int primary key auto_increment not null,
	`chat_id` int not null,
	`author_id` int not null,
	`text` varchar(256),
	
	foreign key (`chat_id`) references `chats`(`chat_id`),
	foreign key (`author_id`) references `users`(`user_id`),
	foreign key (`user_id`) references `users`(`user_id`)
);
