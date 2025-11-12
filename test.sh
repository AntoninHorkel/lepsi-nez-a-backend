#!/bin/sh

echo "Testing create_quiz:"
create_quiz_payload='{
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
quiz_id=$(curl -sS -X POST http://localhost:6767/quiz -H "Content-Type: application/json" -d "$create_quiz_payload")
echo $quiz_id
echo -e "\n"
echo "Testing get_all_quizzes:"
curl -sS -X GET http://localhost:6767/quiz
echo -e "\n"
echo "Testing get_quiz:"
curl -sS -X GET http://localhost:6767/quiz/$quiz_id
echo -e "\n"
echo "Testing update_quiz:"
update_quiz_payload='{
  "name": "Updated Sample Quiz",
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
    }
  ]
}'
# curl -sS -X POST http://localhost:6767/quiz/$quiz_id -H "Content-Type: application/json" -d "$update_quiz_payload"
echo "TODO"
echo -e "\n"
echo "Testing delete_quiz:"
temp_quiz_id=$(curl -sS -X POST http://localhost:6767/quiz -H "Content-Type: application/json" -d "$create_quiz_payload")
curl -sS -X DELETE http://localhost:6767/quiz/$temp_quiz_id
echo -e "\n"
echo "Testing create_instance:"
instance_id=$(curl -sS -X POST http://localhost:6767/quiz/$quiz_id/instance)
echo $instance_id
echo -e "\n"
echo "Testing get_instance:"
curl -sS -X GET http://localhost:6767/quiz/instance/$instance_id
echo -e "\n"
echo "Testing delete_instance:"
temp_instance_id=$(curl -sS -X POST http://localhost:6767/quiz/$quiz_id/instance)
curl -sS -X DELETE http://localhost:6767/quiz/instance/$temp_instance_id
echo -e "\n"
echo "Testing update_instance_state:"
curl -sS -X POST http://localhost:6767/quiz/instance/$instance_id/state -H "Content-Type: text/plain" -d "completed"
echo -e "\n"
echo "Testing post_answer:"
# post_answer_payload='{
#   "questionId": "TODO",
#   "answerId": "TODO",
#   "team": 1
# }'
# curl -sS -X POST http://localhost:6767/quiz/instance/$instance_id/answer -H "Content-Type: application/json" -d "$post_answer_payload"
echo "TODO"
echo -e "\n"
echo "Testing get_all_answers:"
curl -sS -X GET http://localhost:6767/quiz/instance/$instance_id/answer
echo -e "\n"
