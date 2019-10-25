DROP TABLE IF EXISTS `todo`;
CREATE TABLE IF NOT EXISTS `todo` (
`id` int(10) NOT NULL AUTO_INCREMENT,
`name` varchar(50) COLLATE utf8_unicode_ci NOT NULL,
`create_date` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
`update_date` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci AUTO_INCREMENT=1;

DROP TABLE IF EXISTS `user`;
CREATE TABLE IF NOT EXISTS `user` (
`id` int(10) NOT NULL AUTO_INCREMENT,
`email` varchar(100) COLLATE utf8_unicode_ci NOT NULL,
`password` varchar(100) COLLATE utf8_unicode_ci NOT NULL,
`authority_level` int(3) COLLATE utf8_unicode_ci NOT NULL,
`create_date` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
`update_date` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci AUTO_INCREMENT=1;

DROP TABLE IF EXISTS `c_token`;
CREATE TABLE IF NOT EXISTS `c_token` (
`id` int(10) NOT NULL AUTO_INCREMENT,
`user_id` int(10) NOT NULL,
`client_token` varchar(20) COLLATE utf8_unicode_ci NOT NULL UNIQUE,
`client_secret_token` varchar(20) COLLATE utf8_unicode_ci NOT NULL UNIQUE,
PRIMARY KEY (`id`),
FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE,
INDEX (user_id, client_token)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci AUTO_INCREMENT=1;