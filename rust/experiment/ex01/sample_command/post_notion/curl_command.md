```bash
curl 'https://api.notion.com/v1/pages' \
  -H 'Authorization: Bearer '"$secret_key"'' \
  -H "Content-Type: application/json" \
  -H "Notion-Version: 2022-06-28" \
  --data '{
    "parent": { "database_id": "$database_id" },
    "properties": {
        "Name": {
            "title": [
                {
                    "text": {
                        "content": "Tuscan Kale"
                    }
                }
            ]
        },
        "Date": {
            "date": {
                "start": "2022-08-22"
            }
        }
    }
}'
```