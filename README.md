# Web сервис - Умный дом

## База данных

Для создания чистой базы:

```bash
sqlx database create
```

Для применения миграций:

```bash
sqlx migrate run
```

## API. Примеры запросов

### Получить отчет по дому

```bash
curl --request GET \
  --url http://localhost:3000/
```

### Работа с комнатами

1. Список комнат:

    ```bash
    curl --request GET \
    --url http://localhost:3000/rooms
    ```

2. Создать комнату:

    ```bash
    curl --request POST \
    --url http://localhost:3000/rooms \
    --header 'Content-Type: application/json' \
    --data '{
        "name": "first",
        "description": "Первая комната"
    }'
    ```

3. Получить данные по комнате:

    ```bash
    curl --request GET \
    --url http://localhost:3000/rooms/2
    ```

4. Удаление комнаты:

    ```bash
    curl --request DELETE \
    --url http://localhost:3000/rooms/1
    ```

### Работа с умными устройствами

1. Получить данные по устройству:

    ```bash
    curl --request GET \
    --url http://localhost:3000/devices/7
    ```

2. Добавление устройства:

    ```bash
    curl --request POST \
    --url http://localhost:3000/devices \
    --header 'Content-Type: application/json' \
    --data '{
        "room_id": 2,
        "parent_id": 9,
        "device_type": "thermometer",
        "is_on": true
    }'
    ```

3. Удаление устройства:

    ```bash
    curl --request DELETE \
    --url http://localhost:3000/devices/7
    ```

### Пример отчета

```json
{
    "description": "Умный дом",
    "rooms": [
        {
            "room": {
                "id": 2,
                "name": "second",
                "description": "Вторая комната"
            },
            "devices": [
                {
                    "id": 8,
                    "info": "Умный термометр, температура: 4.562084 °C, потребляемая мощность: 20"
                },
                {
                    "id": 9,
                    "info": "Умная розетка: включена, подключено устройств: 0, потребляемая мощность: 0",
                    "devices": [
                        {
                            "id": 10,
                            "info": "Умный термометр, температура: 11.721966 °C, потребляемая мощность: 20"
                        }
                    ]
                }
            ]
        },
        {
            "room": {
                "id": 4,
                "name": "first",
                "description": "Первая комната"
            },
            "devices": []
        }
    ]
}
```
