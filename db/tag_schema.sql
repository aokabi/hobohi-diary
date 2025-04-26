-- タグテーブルの作成
CREATE TABLE IF NOT EXISTS `diary`.`tag` (
  `id` int NOT NULL AUTO_INCREMENT,
  `name` varchar(50) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `name` (`name`)
);

-- エントリとタグの関連テーブル
CREATE TABLE IF NOT EXISTS `diary`.`entry_tag` (
  `entry_id` int NOT NULL,
  `tag_id` int NOT NULL,
  PRIMARY KEY (`entry_id`, `tag_id`),
  FOREIGN KEY (`entry_id`) REFERENCES `entry` (`id`) ON DELETE CASCADE,
  FOREIGN KEY (`tag_id`) REFERENCES `tag` (`id`) ON DELETE CASCADE
);
