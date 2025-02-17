# DataStreamMaster

## Опис
DataStreamMaster - це потужна платформа для управління потоками даних, що включає модулі для збору, обробки, аналізу та візуалізації даних у реальному часі.

## Структура проекту
Проект розділений на кілька шарів для покращення читабельності та підтримуваності коду:

- **Domain**: Основна бізнес-логіка та правила.
- **Application**: Інтерфейси, юзкейси та реалізації для роботи з даними.
- **Infrastructure**: Реалізація деталей інфраструктури, таких як моделі даних, контролери та утиліти.
- **Presentation**: Візуалізація даних та взаємодія з користувачем.

## Встановлення
1. Клонуйте репозиторій:
    ```bash
    git clone <URL репозитарію>
    ```
2. Перейдіть до каталогу проекту:
    ```bash
    cd data_stream_master
    ```
3. Встановіть необхідні залежності:
    ```bash
    cargo build
    ```

## Запуск
Для запуску проекту виконайте наступну команду:
```bash
cargo run
```

## Структура каталогів
```plaintext
data_stream_master/
├── src/
│   ├── domain/
│   │   ├── entities/
│   │   │   ├── DataStream.rs
│   │   │   └── DataRecord.rs
│   │   ├── repositories/
│   │   │   └── DataStreamRepository.rs
│   │   ├── services/
│   │   │   └── DataStreamService.rs
│   │   └── use_cases/
│   │       └── ManageDataStream.rs
│   ├── infrastructure/
│   │   ├── models/
│   │   │   └── DataStreamModel.rs
│   │   ├── repositories/
│   │   │   └── DataStreamRepositoryImpl.rs
│   │   ├── controllers/
│   │   │   └── DataStreamController.rs
│   ├── application/
│       └── main.rs
├── config/
│   └── config.rs
├── Cargo.toml
└── README.md
```

## Опис компонентів
### Domain
- **DataStream.rs**: Клас сутності потоку даних.
- **DataRecord.rs**: Клас сутності запису даних.
- **DataStreamRepository.rs**: Інтерфейс репозиторію потоків даних.

### Application
- **ManageDataStream.rs**: Юзкейс для управління потоками даних.
- **DataStreamService.rs**: Сервіс для управління потоками даних.

### Infrastructure
- **DataStreamModel.rs**: Модель даних потоку даних.
- **DataStreamRepositoryImpl.rs**: Реалізація репозиторію потоків даних.
- **DataStreamController.rs**: Контролер для управління потоками даних.

## Ліцензія
Цей проект ліцензовано під ліцензією MIT. Для отримання додаткової інформації див. файл LICENSE.
