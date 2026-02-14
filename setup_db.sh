#!/bin/bash

# Configuration
PROJECT_ID="walldriyan-r-finance-engine"
REGION="asia-south1"
INSTANCE_NAME="financial-db-prod"
DB_NAME="financial_db"
DB_USER="financial_user"
# රහස් වචනය (Password) අහඹු ලෙස ජනනය කෙරේ (මෙය ආරක්ෂිතයි)
DB_PASS=$(openssl rand -base64 12)

echo "🚀 Starting Cloud SQL Setup for $PROJECT_ID..."

# 1. Cloud SQL API Enable කිරීම
gcloud services enable sqladmin.googleapis.com

# 2. Database Instance එක සෑදීම (PostgreSQL 15, Enterprise Sandbox - Cheap)
# Note: Production සඳහා 'db-custom-1-3840' හෝ ඊට වැඩි එකක් හොඳයි.
# දැනට ලාබම එක (Sandbox) සාදමු.
echo "⏳ creating Database Instance (takes 5-10 mins)..."
gcloud sql instances create $INSTANCE_NAME \
    --database-version=POSTGRES_15 \
    --cpu=1 \
    --memory=3840MB \
    --region=$REGION \
    --root-password=$DB_PASS \
    --project=$PROJECT_ID

# 3. Database එක සෑදීම
echo "📦 Creating Database '$DB_NAME'..."
gcloud sql databases create $DB_NAME --instance=$INSTANCE_NAME

# 4. User කෙනෙක් සෑදීම
echo "👤 Creating User '$DB_USER'..."
gcloud sql users create $DB_USER \
    --instance=$INSTANCE_NAME \
    --password=$DB_PASS

# 5. Connection Info ලබා ගැනීම
CONNECTION_NAME=$(gcloud sql instances describe $INSTANCE_NAME --format="value(connectionName)")

echo "========================================================"
echo "✅ Database Setup Complete!"
echo "--------------------------------------------------------"
echo "📡 Connection Name: $CONNECTION_NAME"
echo "👤 User: $DB_USER"
echo "🔑 Password: $DB_PASS"
echo "🗄️  Database: $DB_NAME"
echo "--------------------------------------------------------"
echo "⚠️  SAVE THIS PASSWORD SECURELY!"
echo "========================================================"

# Save credentials to a file for reference
echo "DATABASE_URL=postgres://$DB_USER:$DB_PASS@/$DB_NAME?host=/cloudsql/$CONNECTION_NAME" > db_credentials.txt
