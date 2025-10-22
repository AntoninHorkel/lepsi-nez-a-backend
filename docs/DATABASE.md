# Database Schema Documentation

## Overview

This document describes the database schema for the Quiz application.

## Database Structure

### Entity Relationship Diagram

```
Quiz (1) ----< (N) Question
  |
  |
  +----------< (N) QuizInstance (1) ----< (N) TeamAnswer >---- (N) Question
```

## Tables

### `quiz`

Stores quiz information.

| Column     | Type                     | Constraints   | Description               |
| ---------- | ------------------------ | ------------- | ------------------------- |
| id         | SERIAL                   | PRIMARY KEY   | Auto-incrementing quiz ID |
| name       | VARCHAR(255)             | NOT NULL      | Name of the quiz          |
| created_at | TIMESTAMP WITH TIME ZONE | DEFAULT NOW() | Creation timestamp        |
| updated_at | TIMESTAMP WITH TIME ZONE | DEFAULT NOW() | Last update timestamp     |

**Relationships:**

- One Quiz has many Questions
- One Quiz has many QuizInstances

---

### `question`

Stores questions belonging to a quiz.

| Column     | Type                     | Constraints             | Description                   |
| ---------- | ------------------------ | ----------------------- | ----------------------------- |
| id         | SERIAL                   | PRIMARY KEY             | Auto-incrementing question ID |
| quiz_id    | INTEGER                  | NOT NULL, FK → quiz(id) | Reference to parent quiz      |
| q          | TEXT                     | NOT NULL                | Question text                 |
| answers    | JSONB                    | NOT NULL                | Array of answer objects       |
| created_at | TIMESTAMP WITH TIME ZONE | DEFAULT NOW()           | Creation timestamp            |
| updated_at | TIMESTAMP WITH TIME ZONE | DEFAULT NOW()           | Last update timestamp         |

**Answer JSON Structure:**

```json
[
	{
		"key": "A",
		"isCorrect": true,
		"content": "Answer text"
	},
	{
		"key": "B",
		"isCorrect": false,
		"content": "Another answer"
	}
]
```

**Relationships:**

- Each Question belongs to one Quiz
- Each Question has many TeamAnswers

**Indexes:**

- `idx_question_quiz_id` on `quiz_id`

---

### `quiz_instance`

Represents an instance of a quiz being taken.

| Column     | Type                     | Constraints               | Description                    |
| ---------- | ------------------------ | ------------------------- | ------------------------------ |
| id         | SERIAL                   | PRIMARY KEY               | Auto-incrementing instance ID  |
| uuid       | UUID                     | NOT NULL, UNIQUE, DEFAULT | Unique identifier for instance |
| quiz_id    | INTEGER                  | NOT NULL, FK → quiz(id)   | Reference to quiz              |
| created_at | TIMESTAMP WITH TIME ZONE | DEFAULT NOW()             | Creation timestamp             |

**Relationships:**

- Each QuizInstance belongs to one Quiz
- Each QuizInstance has many TeamAnswers

**Indexes:**

- `idx_quiz_instance_quiz_id` on `quiz_id`
- `idx_quiz_instance_uuid` on `uuid`

---

### `team_answer`

Stores team answers for quiz instances.

| Column           | Type                     | Constraints                      | Description                   |
| ---------------- | ------------------------ | -------------------------------- | ----------------------------- |
| id               | SERIAL                   | PRIMARY KEY                      | Auto-incrementing answer ID   |
| num              | INTEGER                  | NOT NULL                         | Team number                   |
| ans              | CHAR(1)                  | NOT NULL                         | Answer key (A, B, C, D, etc.) |
| question_id      | INTEGER                  | NOT NULL, FK → question(id)      | Reference to question         |
| quiz_instance_id | INTEGER                  | NOT NULL, FK → quiz_instance(id) | Reference to quiz instance    |
| created_at       | TIMESTAMP WITH TIME ZONE | DEFAULT NOW()                    | Creation timestamp            |

**Constraints:**

- UNIQUE constraint on `(quiz_instance_id, question_id, num)` - ensures one answer per team per question per instance

**Relationships:**

- Each TeamAnswer belongs to one Question
- Each TeamAnswer belongs to one QuizInstance

**Indexes:**

- `idx_team_answer_question_id` on `question_id`
- `idx_team_answer_quiz_instance_id` on `quiz_instance_id`

---

## Triggers

### `update_updated_at_column()`

Automatically updates the `updated_at` timestamp when a row is modified.

Applied to:

- `quiz` table
- `question` table

---

## Sample Data

The migration includes sample data:

- 1 Quiz: "Sample Quiz 1"
- 2 Questions with multiple-choice answers
- 1 Quiz Instance
- 2 Team Answers

---

## API Endpoints

### Get all quizzes

```
GET /api/quizzes
```

Returns an array of all quizzes with their metadata.

### Get questions for a quiz

```
GET /api/questions/:quiz_id
```

Returns all questions for a specific quiz, including their answer options.

---

## Running Migrations

When you start the Docker containers for the first time, the migrations in the `migrations/` directory will automatically run:

```bash
docker-compose up --build
```

To reset the database:

```bash
docker-compose down -v
docker-compose up --build
```
