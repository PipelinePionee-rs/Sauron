import sqlite3
import psycopg2
from datetime import datetime
from dotenv import load_dotenv
import os

def migrate_data():
    load_dotenv()
    # Connect to SQLite
    sqlite_conn = sqlite3.connect('../data/sauron.db')
    sqlite_cursor = sqlite_conn.cursor()

    # Connect to PostgreSQL
    pg_conn = psycopg2.connect(
        dbname=os.getenv("POSTGRES_DB"),
        user=os.getenv("POSTGRES_USER"),
        password=os.getenv("POSTGRES_PASSWORD"),
        host= os.getenv("POSTGRES_HOST", "localhost"),
        port= os.getenv("POSTGRES_PORT", "5432")
    )
    pg_cursor = pg_conn.cursor()

    try:
        # Migrate pages table
        sqlite_cursor.execute("SELECT title, url, language, last_updated, content FROM pages")
        pages = sqlite_cursor.fetchall()
        
        for page in pages:
            pg_cursor.execute("""
                INSERT INTO pages (title, url, language, last_updated, content)
                VALUES (%s, %s, %s, %s, %s)
            """, page)

        # Migrate users table
        sqlite_cursor.execute("SELECT username, email, password FROM users")
        users = sqlite_cursor.fetchall()
        
        for user in users:
            pg_cursor.execute("""
                INSERT INTO users (username, email, password)
                VALUES (%s, %s, %s)
            """, user)

        # Commit the changes
        pg_conn.commit()
        print("Migration completed successfully!")

    except Exception as e:
        print(f"Error during migration: {e}")
        pg_conn.rollback()
    
    finally:
        sqlite_conn.close()
        pg_conn.close()

if __name__ == "__main__":
    migrate_data()