# 🚀 Ultimate Financial Engine (Microservice)

මෙය ඉහළම මට්ටමේ (Enterprise Grade) මූල්‍ය ගණනය කිරීමේ එන්ජිමකි.
Bank-grade accuracy, Audit trails, සහ Multi-DB support මෙහි අන්තර්ගතයි.

## 🛠️ පද්ධතිය ධාවනය කරන ආකාරය (How to Run)

ඔබේ පරිගණකයේ Rust Setup ගැටළු ඇති බැවින්, **Docker** භාවිතා කිරීම වඩාත් සුදුසුයි.

###Option 1: Docker (නිර්දේශිතයි)
ඔබේ පරිගණකයේ Docker Desktop ස්ථාපනය කර තිබිය යුතුය.

1. Terminal එකේ පහත විධානය ක්‍රියාත්මක කරන්න:
   ```bash
   docker-compose up --build
   ```
2. මෙය විනාඩි කිහිපයක් ගත වනු ඇත (පළමු වරට).
3. ඉන්පසු පහත URL හරහා API එකට ඇතුල් විය හැක:
   - **Calculate:** `POST http://localhost:3000/api/v1/calculate`
   - **Refund:** `POST http://localhost:3000/api/v1/refund`

### Option 2: Local Run (Rust ස්ථාපනය කර ඇත්නම්)
*සටහන: ඔබේ Windows පරිගණකයේ 'Visual Studio C++ Build Tools' ස්ථාපනය කර තිබිය යුතුය.*

```bash
# 1. Database එක පණගන්වන්න
docker-compose up -d db redis

# 2. Rust Project එක දුවන්න
cargo run
```

## 🧪 API පරීක්ෂා කිරීම (Testing)

API එක වැඩද කියා බැලීමට පහත `curl` විධානය භාවිතා කළ හැක (Git Bash හෝ Linux Terminal):

```bash
curl -X POST http://localhost:3000/api/v1/calculate \
  -H "Content-Type: application/json" \
  -d '{
    "cart": {
      "items": [
        { "id": "ITEM_001", "price": 100000, "quantity": 2 }
      ]
    },
    "promo_codes": [],
    "jurisdiction": "LK"
  }'
```

## 📁 ව්‍යාපෘතියේ ව්‍යුහය

*   `src/core`: ගණිතමය එන්ජිම (The Brain)
*   `src/api`: Web Server & Routes
*   `src/storage`: Database & Cache Logic
*   `src/security`: WAF & Hack Prevention
*   `src/audit`: Logs & Error Tracking

---
**Developed with Rust 🦀 & Love ❤️**
