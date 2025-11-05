-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE quiz (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL
);

CREATE TABLE question (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    quiz_id UUID NOT NULL REFERENCES quiz(id) ON DELETE CASCADE,
    text TEXT NOT NULL,
    options JSONB NOT NULL,
    correct_answer JSONB
);

CREATE TABLE quiz_instance (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    quiz_id UUID NOT NULL REFERENCES quiz(id) ON DELETE CASCADE,
    state TEXT NOT NULL CHECK (state IN ('active', 'completed', 'paused')),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE team_answer (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    instance_id UUID NOT NULL REFERENCES quiz_instance(id) ON DELETE CASCADE,
    question_id UUID NOT NULL REFERENCES question(id) ON DELETE CASCADE,
    team_num INT NOT NULL CHECK (team_num BETWEEN 1 AND 4),
    answer JSONB NOT NULL,
    submitted_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_question_quiz_id ON question(quiz_id);
CREATE INDEX idx_quiz_instance_quiz_id ON quiz_instance(quiz_id);
CREATE INDEX idx_team_answer_instance_id ON team_answer(instance_id);
CREATE INDEX idx_team_answer_question_id ON team_answer(question_id);
