CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE quizzes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL
);

CREATE TABLE questions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    quiz_id UUID NOT NULL REFERENCES quizzes(id) ON DELETE CASCADE,
    text TEXT NOT NULL
);

CREATE TABLE answers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    question_id UUID NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
    text TEXT NOT NULL,
    is_correct BOOLEAN NOT NULL
);

CREATE TABLE quiz_instances (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    quiz_id UUID NOT NULL REFERENCES quizzes(id) ON DELETE CASCADE,
    state TEXT NOT NULL CHECK (state IN ('active', 'completed', 'paused'))
);

CREATE TABLE team_answers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    instance_id UUID NOT NULL REFERENCES quiz_instances(id) ON DELETE CASCADE,
    question_id UUID NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
    answer_id UUID NOT NULL REFERENCES answers(id) ON DELETE CASCADE,
    team INTEGER NOT NULL,
    submitted_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for performance
CREATE INDEX idx_questions_quiz_id ON questions(quiz_id);
CREATE INDEX idx_answers_question_id ON answers(question_id);
CREATE INDEX idx_quiz_instances_quiz_id ON quiz_instances(quiz_id);
CREATE INDEX idx_team_answers_instance_id ON team_answers(instance_id);
CREATE INDEX idx_team_answers_question_id ON team_answers(question_id);
CREATE INDEX idx_team_answers_answer_id ON team_answers(answer_id);
