#!/bin/bash

# ඔබේ Project ID එක (Screenshot එකෙන් ගත්)
PROJECT_ID="walldriyan-r-finance-engine"

# Project Number එක ලබා ගැනීම (Auto-detect)
echo "🔍 Finding Project Number..."
PROJECT_NUMBER=$(gcloud projects describe $PROJECT_ID --format="value(projectNumber)")

if [ -z "$PROJECT_NUMBER" ]; then
    echo "❌ Project Number සොයාගත නොහැක. කරුණාකර 'gcloud auth login' වී ඇති බව තහවුරු කරගන්න."
    exit 1
fi

echo "✅ Project Number: $PROJECT_NUMBER"

# Cloud Build Service Account විද්‍යුත් ලිපිනය
CB_SA="$PROJECT_NUMBER@cloudbuild.gserviceaccount.com"

# 1. Cloud Run Admin අවසරය ලබා දීම
echo "🛠️ Adding 'Cloud Run Admin' role..."
gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="serviceAccount:$CB_SA" \
    --role="roles/run.admin"

# 2. Service Account User අවසරය ලබා දීම
echo "🛠️ Adding 'Service Account User' role..."
gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="serviceAccount:$CB_SA" \
    --role="roles/iam.serviceAccountUser"

echo "🎉 Success! Permissions Fixed. දැන් නැවත Trigger එක සාදන්න."
