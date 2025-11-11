#!/bin/sh

echo "Testing create_quiz:"
payload='{
  "name": "Sample Quiz",
  "questions": [
    {
      "text": "What is the capital of France?",
      "answers": [
        {
          "text": "Paris",
          "isCorrect": true
        },
        {
          "text": "London",
          "isCorrect": false
        },
        {
          "text": "Berlin",
          "isCorrect": false
        }
      ]
    },
    {
      "text": "What is 9 + 10?",
      "answers": [
        {
          "text": "19",
          "isCorrect": false
        },
        {
          "text": "21",
          "isCorrect": true
        },
        {
          "text": "67",
          "isCorrect": false
        }
      ]
    }
  ]
}'
curl -X POST http://localhost:6767/quiz -H "Content-Type: application/json" -d "$payload"
echo "\n\n"
echo "Testing get_all_quizzes:"
curl -X GET http://localhost:6767/quiz
echo "\n\n"
echo "Testing get_guiz:"
curl -X GET http://localhost:6767/quiz/b6016b8f-d1ea-418f-a7ee-5227902520d0 # TODO: Get id from create_quiz result.
