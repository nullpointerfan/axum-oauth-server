# Axum OAuth2 Server Template

Этот шаблон предоставляет базовый сервер на фреймворке Axum с поддержкой аутентификации через OAuth2 (Google).

## Функциональность

- **OAuth2 аутентификация**: Интеграция с Google OAuth2 для аутентификации пользователей
- **Защищенные маршруты**: Пример защищенного маршрута, требующего Bearer токен
- **Сессии**: Простое управление сессиями с использованием session_id
- **CORS**: Поддержка CORS для веб-клиентов

## Структура проекта

```
axum-oauth-server/
├── Cargo.toml          # Зависимости проекта
├── src/
│   └── main.rs         # Основной код сервера
└── README.md           # Этот файл
```

## Настройка

1. **Создайте проект Google OAuth2**:
   - Перейдите в [Google Cloud Console](https://console.cloud.google.com/)
   - Создайте новый проект или выберите существующий
   - Включите Google+ API
   - Создайте учетные данные OAuth2 (Client ID и Client Secret)
   - Добавьте `http://localhost:3000/auth/callback` в Authorized redirect URIs

2. **Настройте переменные окружения**:
   - Скопируйте `.env.example` в `.env`
   - Заполните реальные значения для `GOOGLE_CLIENT_ID` и `GOOGLE_CLIENT_SECRET`

## Запуск

```bash
cargo run
```

Сервер будет запущен на `http://localhost:3000`.

## API Endpoints

### `GET /auth/login`
Возвращает URL для перенаправления пользователя на страницу аутентификации Google.

**Ответ:**
```json
{
  "auth_url": "https://accounts.google.com/o/oauth2/auth?..."
}
```

### `GET /auth/callback?code=<code>&state=<state>`
Обрабатывает callback от Google OAuth2 и возвращает session_id.

**Ответ:**
```json
{
  "access_token": "session_id"
}
```

### `GET /protected`
Защищенный маршрут, требующий Bearer токен в заголовке Authorization.

**Заголовки:**
```
Authorization: Bearer <session_id>
```

**Ответ (успешный):**
```json
{
  "message": "Access granted"
}
```

**Ответ (ошибка):**
```json
{
  "error": "Unauthorized"
}
```

## Использование

1. Получите auth_url через `/auth/login`
2. Перенаправьте пользователя на этот URL
3. После аутентификации Google перенаправит на `/auth/callback`
4. Используйте полученный session_id как Bearer токен для доступа к защищенным маршрутам

## Зависимости

- `axum`: Веб-фреймворк
- `tokio`: Асинхронный runtime
- `oauth2`: Библиотека для OAuth2
- `serde`: Сериализация/десериализация
- `tower-http`: HTTP middleware
- `uuid`: Генерация уникальных идентификаторов

## Расширение

Этот шаблон можно расширить добавлением:
- Поддержки других OAuth2 провайдеров (GitHub, Facebook, etc.)
- JWT токенов вместо session_id
- Базы данных для хранения сессий
- Ролей и разрешений пользователей
- Дополнительных защищенных маршрутов