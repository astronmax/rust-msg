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
