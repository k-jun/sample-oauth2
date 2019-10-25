DROP TABLE IF EXISTS `user`;
CREATE TABLE IF NOT EXISTS `user` (
`id` int(10) NOT NULL AUTO_INCREMENT,
`email` varchar(100) COLLATE utf8_unicode_ci NOT NULL,
`password` varchar(100) COLLATE utf8_unicode_ci NOT NULL,
`authority_level` int(3) COLLATE utf8_unicode_ci NOT NULL,
`created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
`updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci AUTO_INCREMENT=1;

DROP TABLE IF EXISTS `client_token`;
CREATE TABLE IF NOT EXISTS `client_token` (
`id` int(10) NOT NULL AUTO_INCREMENT,
`user_id` int(10) NOT NULL,
`token` varchar(20) COLLATE utf8_unicode_ci NOT NULL UNIQUE,
`secret_token` varchar(20) COLLATE utf8_unicode_ci NOT NULL UNIQUE,
PRIMARY KEY (`id`),
FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE,
INDEX (user_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci AUTO_INCREMENT=1;

DROP TABLE IF EXISTS `temp_token`;
CREATE TABLE IF NOT EXISTS `temp_token` (
`id` int(10) NOT NULL AUTO_INCREMENT,
`user_id` int(10) NOT NULL,
`client_token_id` int(10) NOT NULL,
`token` varchar(10) COLLATE utf8_unicode_ci NOT NULL,
`created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
PRIMARY KEY (`id`),
FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE,
FOREIGN KEY (client_token_id) REFERENCES client_token(id) ON DELETE CASCADE,
INDEX (user_id, client_token_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci AUTO_INCREMENT=1;

DROP TABLE IF EXISTS `access_token`;
CREATE TABLE IF NOT EXISTS `access_token` (
`id` int(10) NOT NULL AUTO_INCREMENT,
`token` varchar(10) COLLATE utf8_unicode_ci NOT NULL,
`refresh_token` varchar(10) COLLATE utf8_unicode_ci NOT NULL,
`created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci AUTO_INCREMENT=1;