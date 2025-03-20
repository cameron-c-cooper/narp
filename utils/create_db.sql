SELECT 'CREATE DATABASE narp_db'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname='narp_db')\gexec
