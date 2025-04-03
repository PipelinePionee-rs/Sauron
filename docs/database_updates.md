### changes to datbase

added should_reset column to users table. 

ALTER TABLE users ADD COLUMN should_reset INTEGER;

and set all occurences of it to true (1);

UPDATE users SET should_reset = 1;

this facilitates checking on should_reset, when logging, and we can properly prompt users to change password.
