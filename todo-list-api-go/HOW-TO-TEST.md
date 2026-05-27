# Как писать тесты в Go: Стандарты и практики крупных IT-компаний

Качественное тестирование — один из столпов коммерческой разработки на Go. В крупных технологических компаниях (таких как Google, Uber, Avito, Ozon, Яндекс) к тестам предъявляются строгие требования: они должны быть надежными, быстрыми, легко читаемыми и простыми в поддержке.

Ниже представлено подробное руководство, основанное на анализе вашего проекта **Todo List API**, показывающее, как спроектировать и написать тестовую систему по лучшим стандартам индустрии.

---

## 📌 1. Философия тестирования в Go

Go-сообщество придерживается нескольких золотых правил:
1. **Простота важнее магии:** Избегайте сложных фреймворков для тестирования (вроде ginkgo/gomega). Стандартный пакет `testing` в сочетании с библиотеками утверждений (assertions) — лучший выбор.
2. **Table-Driven Tests (Табличные тесты):** Официальный стандарт Go для минимизации дублирования кода.
3. **Явность (Explicit is better than implicit):** В тестах не должно быть скрытой магии или неявных глобальных состояний.
4. **Тесты как документация:** Хороший тест должен читаться как спецификация к поведению функции.

---

## 🛠️ 2. Архитектурный рефакторинг: Внедрение зависимостей (Dependency Injection)

В вашем текущем коде структура `Env` жестко привязана к конкретной реализации базы данных `*DB`:

```go
type Env struct {
    db *DB // Прямая зависимость от SQLite
}
```

Это делает **Unit-тестирование обработчиков (handlers) невозможным**, так как для любого теста потребуется запущенный файл базы данных `todo.db`.

### Как делают в хороших компаниях:
Выделяют интерфейс хранилища (`Repository`). Это позволяет легко подменять реальную БД на **Mock (заглушку)** во время юнит-тестирования.

#### Шаг 1. Опишем интерфейсы (например, в `repository.go` или прямо в `models.go`):

```go
package main

type UserRepository interface {
	Register(name string, email string, password []byte) (*DbUser, error)
	Login(email string, password []byte) (*DbUser, error)
}

type TodoRepository interface {
	AddTodo(title, description string, userId int) (*DbTodo, error)
	DeleteTodo(todoID int, userID int) error
	UpdateTodo(id int, title, description string, completed bool, userId int) error
	GetTodos(userId int) ([]DbTodo, error)
}

// Объединяющий интерфейс для удобства
type Storage interface {
	UserRepository
	TodoRepository
}
```

#### Шаг 2. Изменим `Env` так, чтобы он работал с интерфейсом:

```go
type Env struct {
	db Storage // Теперь мы зависим от интерфейса, а не от структуры!
}

func NewEnv(db Storage) *Env {
	return &Env{db: db}
}
```
*Ваш текущий struct `DB` автоматически удовлетворяет интерфейсу `Storage`, поэтому в `main.go` ничего менять не придется!*

---

## 📊 3. Table-Driven Tests (Табличные тесты)

Это паттерн №1 в Go. Рассмотрим его на примере тестирования JWT-модуля (`jwt.go`). 

Создадим файл `jwt_test.go`:

```go
package main_test

import (
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	
	. "nobel/todo-api" // Импортируем тестируемый пакет
)

func TestVerifyJwt(t *testing.T) {
	// 1. Описываем структуру тест-кейса
	type testCase struct {
		name          string
		tokenFunc     func() string
		wantErr       bool
		expectedEmail string
	}

	// 2. Объявляем таблицу тестов
	tests := []testCase{
		{
			name: "Успешная валидация валидного токена",
			tokenFunc: func() string {
				token, _ := GenerateJwt(DbUser{ID: 1, User: User{Email: "test@example.com"}})
				return token
			},
			wantErr:       false,
			expectedEmail: "test@example.com",
		},
		{
			name: "Ошибка при невалидной подписи",
			tokenFunc: func() string {
				return "invalid.token.string"
			},
			wantErr: true,
		},
	}

	// 3. Запускаем тесты в цикле
	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			token := tc.tokenFunc()
			claims, err := VerifyJwt(token)

			if tc.wantErr {
				assert.Error(t, err)
				assert.Nil(t, claims)
			} else {
				require.NoError(t, err)
				require.NotNil(t, claims)
				
				email := (*claims)["email"].(string)
				assert.Equal(t, tc.expectedEmail, email)
			}
		})
	}
}
```

> [!TIP]
> **Почему это круто?** 
> Если вам понадобится протестировать просроченный токен (expired token) или токен без ID пользователя, вы просто добавите **одну строчку** в массив `tests`, не дублируя логику вызовов и проверок.

---

## 🎭 4. Mocking (Юнит-тесты без базы данных)

Для тестирования хэндлеров нам нужно имитировать поведение базы данных. Мы напишем простой ручной mock (или сгенерируем его с помощью библиотеки `mockery`).

Создадим файл `handlers_test.go`:

```go
package main_test

import (
	"bytes"
	"encoding/json"
	"errors"
	"net/http/httptest"
	"testing"

	"github.com/gofiber/fiber/v3"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
	
	. "nobel/todo-api"
)

// Шаг 1. Создаем Mock для нашего интерфейса хранилища
type MockStorage struct {
	mock.Mock
}

func (m *MockStorage) Register(name string, email string, password []byte) (*DbUser, error) {
	args := m.Called(name, email, password)
	if args.Get(0) == nil {
		return nil, args.Error(1)
	}
	return args.Get(0).(*DbUser), args.Error(1)
}

func (m *MockStorage) Login(email string, password []byte) (*DbUser, error) {
	args := m.Called(email, password)
	if args.Get(0) == nil {
		return nil, args.Error(1)
	}
	return args.Get(0).(*DbUser), args.Error(1)
}

func (m *MockStorage) AddTodo(title, description string, userId int) (*DbTodo, error) {
	args := m.Called(title, description, userId)
	if args.Get(0) == nil {
		return nil, args.Error(1)
	}
	return args.Get(0).(*DbTodo), args.Error(1)
}

// Заглушки для неиспользуемых в конкретном тесте методов
func (m *MockStorage) DeleteTodo(todoID int, userID int) error { return nil }
func (m *MockStorage) UpdateTodo(id int, title, description string, completed bool, userId int) error { return nil }
func (m *MockStorage) GetTodos(userId int) ([]DbTodo, error) { return nil, nil }

// Шаг 2. Пишем тест для обработчика создания задачи
func TestCreateTodoHandler(t *testing.T) {
	t.Run("Успешное создание Todo", func(t *testing.T) {
		// Инициализируем Fiber и наш Mock
		app := fiber.New()
		mockStore := new(MockStorage)
		env := NewEnv(mockStore)

		// Настраиваем поведение заглушки (Mock Expectation)
		expectedTodo := &DbTodo{
			ID: 42,
			Todo: Todo{
				Title:       "Купить молоко",
				Description: "Вкусвилл, 3.2%",
				Completed:   false,
				UserID:      100,
			},
		}
		mockStore.On("AddTodo", "Купить молоко", "Вкусвилл, 3.2%", 100).Return(expectedTodo, nil)

		// Регистрируем маршрут с тестовым Middleware, эмулирующим авторизованного пользователя
		app.Post("/todos", func(c fiber.Ctx) error {
			c.Locals("userId", 100) // Эмулируем AuthMiddleware
			return c.Next()
		}, env.CreateTodoHandler)

		// Формируем HTTP-запрос
		reqBody, _ := json.Marshal(Todo{
			Title:       "Купить молоко",
			Description: "Вкусвилл, 3.2%",
		})
		req := httptest.NewRequest("POST", "/todos", bytes.NewBuffer(reqBody))
		req.Header.Set("Content-Type", "application/json")

		// Вызываем роутер Fiber напрямую (без запуска TCP-сервера)
		resp, err := app.Test(req)
		require.NoError(t, err)
		assert.Equal(t, fiber.StatusCreated, resp.StatusCode)

		// Проверяем тело ответа
		var body map[string]interface{}
		err = json.NewDecoder(resp.Body).Decode(&body)
		require.NoError(t, err)
		
		assert.Equal(t, "Todo created successfully", body["message"])
		
		// Проверяем, что все ожидания от Mock-объекта были выполнены
		mockStore.AssertExpectations(t)
	})

	t.Run("Ошибка 400 при пустом Title", func(t *testing.T) {
		app := fiber.New()
		mockStore := new(MockStorage)
		env := NewEnv(mockStore)

		app.Post("/todos", env.CreateTodoHandler)

		reqBody, _ := json.Marshal(Todo{
			Title:       "", // Пустой заголовок
			Description: "Описание",
		})
		req := httptest.NewRequest("POST", "/todos", bytes.NewBuffer(reqBody))
		req.Header.Set("Content-Type", "application/json")

		resp, err := app.Test(req)
		require.NoError(t, err)
		assert.Equal(t, fiber.StatusBadRequest, resp.StatusCode)
	})
}
```

---

## 🗄️ 5. Integration Tests (Интеграционные тесты с SQLite в памяти)

Юнит-тесты проверяют бизнес-логику в изоляции, но они не гарантируют, что ваши SQL-запросы написаны правильно. Для этого нужны интеграционные тесты.

Так как вы используете **SQLite**, мы можем запустить тесты на **In-Memory базе данных** (`:memory:`). Она работает в оперативной памяти, запускается за миллисекунды, и сама очищается после закрытия соединения!

Создадим файл `db_test.go`:

```go
package main_test

import (
	"database/sql"
	"testing"

	"github.com/stretchr/assert"
	"github.com/stretchr/testify/require"
	_ "modernc.org/sqlite"

	. "nobel/todo-api"
)

// Вспомогательная функция для поднятия тестовой БД
func setupTestDB(t *testing.T) *DB {
	t.Helper() // Указывает, что функция является вспомогательной
	
	// Подключаемся к SQLite в памяти
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	require.NoError(t, err)
	
	db := &DB{DB: sqliteDB}
	db.InitTables() // Создаем схему
	
	// Автоматическое закрытие соединения по окончании теста
	t.Cleanup(func() {
		db.Close()
	})
	
	return db
}

func TestDB_AddAndGetTodo(t *testing.T) {
	db := setupTestDB(t)

	// 1. Создаем тестового пользователя
	user, err := db.Register("Иван", "ivan@example.com", []byte("hash"))
	require.NoError(t, err)
	require.NotNil(t, user)

	// 2. Проверяем добавление задачи
	todo, err := db.AddTodo("Купить хлеб", "Бородинский", user.ID)
	require.NoError(t, err)
	assert.Equal(t, "Купить хлеб", todo.Title)
	assert.Equal(t, user.ID, todo.UserID)

	// 3. Проверяем получение списка задач
	todos, err := db.GetTodos(user.ID)
	require.NoError(t, err)
	require.Len(t, todos, 1)
	assert.Equal(t, todo.ID, todos[0].ID)
	assert.Equal(t, "Купить хлеб", todos[0].Title)
}
```

---

## 📦 6. Пакеты и инструменты, которые нужно использовать

В Go-индустрии стандартным набором инструментов тестирования являются:

1. **`github.com/stretchr/testify`** — стандарт де-факто для ассертов:
   - `assert`: для некритичных проверок (тест продолжается в случае неудачи).
   - `require`: для критичных проверок (тест падает сразу, например, если `err != nil`).
2. **`go test` (встроенные флаги):**
   - `go test -v ./...` — запустить все тесты в режиме подробного вывода.
   - `go test -run TestFunctionName ./...` — запустить конкретный тест.
   - `go test -cover ./...` — проверить процент покрытия тестами.
   - `go test -race ./...` — **КРИТИЧЕСКИ ВАЖНО!** Детектор гонки данных (Data Race Detector). В хороших компаниях CI/CD никогда не пропустит код, не прошедший проверку с флагом `-race`.

---

## 📈 7. Как измерить покрытие тестами (Coverage)

В крупных компаниях нормой считается покрытие кода тестами на уровне **70–80%** (особенно для бизнес-логики).

Вы можете сгенерировать интерактивный HTML-отчет о покрытии:

```bash
# 1. Запустить тесты и записать профиль покрытия
go test -coverprofile=coverage.out ./...

# 2. Открыть отчет в браузере (подсветит зеленым покрытые строки, красным - пропущенные)
go tool cover -html=coverage.out
```

---

## 🏆 Чек-лист «Тестирование на уровне Senior Go Developer»

- [ ] Файлы тестов лежат рядом с тестируемым кодом и имеют суффикс `_test.go`.
- [ ] Используется изоляция пакета: тесты пишутся в `package main_test`, чтобы проверять только внешнее API.
- [ ] Все внешние вызовы (БД, сторонние API, отправка почты) скрыты за интерфейсами и покрыты Mock-объектами.
- [ ] В тестах нет глобального состояния. Каждый тест запускается в независимой среде.
- [ ] Для интеграционных тестов с БД используется `t.Cleanup()` для очистки данных.
- [ ] В тестах не используются `panic`, `log.Fatal` или `os.Exit`. Только `t.Error()`, `t.Fatal()` или `testify`.
- [ ] Тесты запускаются без ошибок с флагом `-race`.
