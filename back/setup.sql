-- Set Settings
SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
START TRANSACTION;
SET time_zone = "+02:00";


-- Make table verified_tokens
CREATE TABLE `businesses` (
  `id` int(11) NOT NULL,
  `org` int(20) NOT NULL,
  `notes` text NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- Set primary keys
ALTER TABLE `businesses`
  ADD PRIMARY KEY (`id`);


-- Set auto increment
ALTER TABLE `businesses`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=1;


-- End
COMMIT;