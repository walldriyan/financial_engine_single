#!/bin/bash
# ----------------------------------------------------------------------
# 🚀 Switch Financial Engine to Supabase (Or Any Postgres)
# ----------------------------------------------------------------------

# Ask for the new Database URL (from Supabase Dashboard)
echo "Paste your Supabase Connection URL:"
echo "(Format: postgres://postgres.xxxx:yourpassword@aws-0-ap-south-1.pooler.supabase.com:5432/postgres)"
read -p "Database URL: " DB_URL

if [ -z "$DB_URL" ]; then
    echo "❌ Error: Database URL cannot be empty."
    exit 1
fi

echo "=========================================================="
echo "🔄 Updating Cloud Run Service to use Supabase..."
echo "=========================================================="

# 1. Update Env Vars & Remove Google Cloud SQL binding
# Note: This command removes the '--add-cloudsql-instances' flag effectively.
gcloud run services update walldriyan-r-financeeal-engine \
    --region asia-south1 \
    --remove-cloudsql-instances walldriyan-r-finance-engine:asia-south1:financial-db-prod \
    --set-env-vars DATABASE_URL="$DB_URL"

echo "✅ App Updated Successfully!"
echo "----------------------------------------------------------"

# 2. Ask to Delete Google Cloud SQL (Save Money)
read -p "💰 Do you want to DELETE the old Google Cloud SQL instance to save money? (y/n): " CONFIRM
if [[ "$CONFIRM" == "y" || "$CONFIRM" == "Y" ]]; then
    echo "🗑️  Deleting Cloud SQL Instance 'financial-db-prod'..."
    gcloud sql instances delete financial-db-prod --quiet
    echo "✅ Old Database Deleted. No more bills!"
else
    echo "ℹ️  Old Database Kept. Remember to delete it later if not used."
fi

echo "🎉 Migration to Supabase Complete!"
