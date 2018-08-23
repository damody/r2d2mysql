### mysql
use mysql 8.0
```sql
CREATE DATABASE eat;
CREATE USER 'eat'@'localhost' IDENTIFIED BY 'eateat';
GRANT ALL PRIVILEGES ON eat.* TO 'eat'@'localhost';
```
