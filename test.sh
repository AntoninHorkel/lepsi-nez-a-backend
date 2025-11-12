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
quiz_id=$(curl -X POST http://localhost:6767/quiz -H "Content-Type: application/json" -d "$payload")
echo $quiz_id
echo -e "\n"
echo "Testing get_all_quizzes:"
curl -X GET http://localhost:6767/quiz
echo -e "\n"
echo "Testing get_guiz:"
curl -X GET http://localhost:6767/quiz/$quiz_id
