# API Reference

## Base URL

```
http://localhost:6767
```

## Endpoints

### Health Check

#### `GET /health`

Check the health of the application and database connection.

**Response:**

```json
{
	"status": "healthy"
}
```

**Status Codes:**

- `200 OK` - Service is healthy
- `503 Service Unavailable` - Database connection failed

---

### Quizzes

#### `GET /api/quizzes`

Get all quizzes.

**Response:**

```json
[
	{
		"id": 1,
		"name": "Sample Quiz 1",
		"created_at": "2025-10-22T10:00:00Z",
		"updated_at": "2025-10-22T10:00:00Z"
	}
]
```

**Status Codes:**

- `200 OK` - Success

**Example:**

```bash
curl http://localhost:6767/api/quizzes
```

---

### Questions

#### `GET /api/questions/:quiz_id`

Get all questions for a specific quiz.

**Parameters:**

- `quiz_id` (path) - The ID of the quiz

**Response:**

```json
[
	{
		"id": 1,
		"quiz_id": 1,
		"q": "What is the capital of France?",
		"answers": [
			{
				"key": "A",
				"isCorrect": true,
				"content": "Paris"
			},
			{
				"key": "B",
				"isCorrect": false,
				"content": "London"
			}
		],
		"created_at": "2025-10-22T10:00:00Z",
		"updated_at": "2025-10-22T10:00:00Z"
	}
]
```

**Status Codes:**

- `200 OK` - Success
- `500 Internal Server Error` - Database error

**Example:**

```bash
curl http://localhost:6767/api/questions/1
```

---

## Testing with curl

### Check health

```bash
curl http://localhost:6767/health
```

### Get all quizzes

```bash
curl http://localhost:6767/api/quizzes | jq
```

### Get questions for quiz #1

```bash
curl http://localhost:6767/api/questions/1 | jq
```

---

## Future Endpoints (To be implemented)

- `POST /api/quizzes` - Create a new quiz
- `POST /api/questions` - Create a new question
- `POST /api/quiz-instances` - Create a new quiz instance
- `POST /api/team-answers` - Submit a team answer
- `GET /api/quiz-instances/:uuid` - Get quiz instance by UUID
- `GET /api/quiz-instances/:uuid/results` - Get results for a quiz instance
