#!/bin/sh

curl --location 'http://127.0.0.1:3000/api/v1/users' \
--header 'Content-Type: application/json' \
--data '{
    "id": "497f6eca-6276-4993-bfeb-53cbbbba6f00",
    "name": "HackerBot",
    "status": "Chilling 😎",
    "from_system": "791718bb-1a61-42e2-8c03-cf9d1f325d04",
    "avatar": "http://example.com"
  }'

sleep 1

curl --location 'http://127.0.0.1:3000/api/v1/users' \
--header 'Content-Type: application/json' \
--data '{
    "id": "887f6eca-6276-4993-bfeb-53cbbbba6f25",
    "name": "HackerBot3",
    "status": "Chilling 😎😎😎😎",
    "from_system": "791718bb-1a61-42e2-8c03-cf9d1f325d04",
    "avatar": "http://example.com"
  }'

sleep 1

curl --location 'http://127.0.0.1:3000/internal/channels' \
--header 'Content-Type: application/json' \
--data '{
    "id": "497f6eca-6276-4993-bfeb-53cbbbba6f08",
    "model": {
        "id": "497f6eca-6276-4993-bfeb-53cbbbba6f08",
        "name": "Channel 1",
        "icon": "https://brand.trifork.com/wp-content/themes/trifork-brand/img/logo-trifork-dark.svg",
        "description": "A channel for chatting with awesome Triforkers! ",
        "visibility": true,
        "size": 50
    },
    "owner_id": "597f6eca-6276-4993-bfeb-53cbbbba6f25"
}'

sleep 1

curl --location 'http://127.0.0.1:3000/internal/channels' \
--header 'Content-Type: application/json' \
--data '{
    "id": "99996eca-6276-4993-bfeb-53cbbbba6f09",
    "model": {
        "id": "497f6eca-6276-4993-bfeb-53cbbbba6f08",
        "name": "Channel 1",
        "icon": "https://brand.trifork.com/wp-content/themes/trifork-brand/img/logo-trifork-dark.svg",
        "description": "A channel for chatting with awesome Triforkers! ",
        "visibility": true,
        "size": 50
    },
    "owner_id": "597f6eca-6276-4993-bfeb-53cbbbba6f25"
}'


sleep 1

curl --request POST \
  --url http://127.0.0.1:8000/api/v1/channels/channel_id/messages \
  --header 'Content-Type: application/json' \
  --data '{
  "id": "497f6eca-6276-4993-bfeb-53cbbbba6f08",
  "timestamp": "2019-08-24T14:15:22Z",
  "message": {
    "html": "<marquee>Hackerdays 2023</marquee>",
    "text": "Hackerdays 2023"
  },
  "from_user": "8b1114d6-dbc3-4464-9f6a-d8d3099c5ba5"
}'