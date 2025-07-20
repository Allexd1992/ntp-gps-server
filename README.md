# NTP GPS Server - Монорепозиторий

Этот проект представляет собой монорепозиторий для системы NTP GPS сервера, состоящий из нескольких взаимосвязанных сервисов.

## Структура проекта

```
ntp-gps-server/
├── backend/                 # Rust backend сервер
├── login-service/          # Python сервис для детекции логинов
├── network-service/        # Python сервис для управления сетью
├── oled-connector/         # Python сервис для OLED дисплея
├── rtc-control-service/    # Python сервис для управления RTC
├── web-gui/               # React TypeScript веб-интерфейс
└── oled_i2c_connector/    # Дополнительный OLED модуль
```

## Компоненты

### Backend (Rust)
- NTP сервер с GPS синхронизацией
- REST API для управления системой
- Swagger документация
- Система диагностики

### Login Service (Python)
- Детекция пользовательских логинов
- Системный сервис для мониторинга

### Network Service (Python)
- Управление сетевыми настройками
- Мониторинг сетевого состояния

### OLED Connector (Python)
- Управление OLED дисплеем
- Отображение системной информации

### RTC Control Service (Python)
- Управление Real-Time Clock
- Синхронизация времени

### Web GUI (React + TypeScript)
- Современный веб-интерфейс
- Redux для управления состоянием
- Компонентная архитектура

## Установка и запуск

### Требования
- Rust (для backend)
- Python 3.x (для Python сервисов)
- Node.js (для web-gui)
- Git

### Быстрый старт

1. Клонируйте репозиторий:
```bash
git clone <repository-url>
cd ntp-gps-server
```

2. Запустите backend:
```bash
cd backend
cargo run
```

3. Запустите web-gui:
```bash
cd web-gui
npm install
npm start
```

4. Запустите Python сервисы:
```bash
cd login-service
python service.py

cd ../network-service
python service.py

cd ../oled-connector
python service.py

cd ../rtc-control-service
python service.py
```

## Разработка

Этот монорепозиторий позволяет:
- Централизованно управлять всеми компонентами системы
- Обеспечивать согласованность версий
- Упрощать развертывание
- Улучшать координацию разработки

## Лицензия

[Укажите лицензию проекта]
