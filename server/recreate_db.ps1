dropdb -U "$($env:USERNAME)" -h 127.0.0.1 -p 5432 db
createdb -U "$($env:USERNAME)" -h 127.0.0.1 -p 5432 db
diesel database setup --database-url "postgres://$($env:USERNAME)@localhost:5432/db"