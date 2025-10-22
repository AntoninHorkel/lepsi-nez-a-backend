-- Insert sample quiz
INSERT INTO quiz (name) VALUES ('Sample Quiz 1');

-- Insert sample questions
INSERT INTO question (quiz_id, q, answers) VALUES 
(1, 'What is the capital of France?', '[
    {"key": "A", "isCorrect": true, "content": "Paris"},
    {"key": "B", "isCorrect": false, "content": "London"},
    {"key": "C", "isCorrect": false, "content": "Berlin"},
    {"key": "D", "isCorrect": false, "content": "Madrid"}
]'::jsonb),
(1, 'What is 2 + 2?', '[
    {"key": "A", "isCorrect": false, "content": "3"},
    {"key": "B", "isCorrect": true, "content": "4"},
    {"key": "C", "isCorrect": false, "content": "5"},
    {"key": "D", "isCorrect": false, "content": "6"}
]'::jsonb);

-- Insert sample quiz instance
INSERT INTO quiz_instance (quiz_id) VALUES (1);

-- Insert sample team answers
INSERT INTO team_answer (num, ans, question_id, quiz_instance_id) VALUES
(1, 'A', 1, 1),
(1, 'B', 2, 1);
