#!/bin/bash

# Database setup script for local development

set -e

DB_NAME="lepsi_nez_a"
DB_USER="postgres"
DB_PASSWORD="postgres"
DB_HOST="localhost"
DB_PORT="5432"

echo "🚀 Setting up database: $DB_NAME"

# Check if PostgreSQL is running
if ! pg_isready -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" &>/dev/null; then
    echo "❌ PostgreSQL is not running on $DB_HOST:$DB_PORT"
    echo "   Start it with: docker-compose up postgres -d"
    exit 1
fi

echo "✅ PostgreSQL is running"

# Check if database exists
if psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -lqt | cut -d \| -f 1 | grep -qw "$DB_NAME"; then
    echo "⚠️  Database '$DB_NAME' already exists"
    read -p "Do you want to drop and recreate it? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "🗑️  Dropping database..."
        PGPASSWORD="$DB_PASSWORD" dropdb -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" "$DB_NAME"
    else
        echo "Keeping existing database"
        exit 0
    fi
fi

# Create database
echo "📦 Creating database..."
PGPASSWORD="$DB_PASSWORD" createdb -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" "$DB_NAME"

# Run migrations
echo "🔨 Running migrations..."
for migration in migrations/*.sql; do
    echo "   Executing: $migration"
    PGPASSWORD="$DB_PASSWORD" psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -f "$migration"
done

echo "✨ Database setup complete!"
echo ""
echo "Connection string:"
echo "postgres://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME"
