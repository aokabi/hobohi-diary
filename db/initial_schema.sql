CREATE DATABASE IF NOT EXISTS `diary`;

CREATE TABLE IF NOT EXISTS `diary`.`entry` (
  `id` int NOT NULL AUTO_INCREMENT,
  `content` text NOT NULL,
  `datetime` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3;