-- Create UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Quiz table
CREATE TABLE quiz (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Question table
CREATE TABLE question (
    id SERIAL PRIMARY KEY,
    quiz_id INTEGER NOT NULL REFERENCES quiz(id) ON DELETE CASCADE,
    q TEXT NOT NULL,
    answers JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- QuizInstance table
CREATE TABLE quiz_instance (
    id SERIAL PRIMARY KEY,
    uuid UUID NOT NULL UNIQUE DEFAULT uuid_generate_v4(),
    quiz_id INTEGER NOT NULL REFERENCES quiz(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- TeamAnswer table
CREATE TABLE team_answer (
    id SERIAL PRIMARY KEY,
    num INTEGER NOT NULL,
    ans CHAR(1) NOT NULL,
    question_id INTEGER NOT NULL REFERENCES question(id) ON DELETE CASCADE,
    quiz_instance_id INTEGER NOT NULL REFERENCES quiz_instance(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(quiz_instance_id, question_id, num)
);

-- Create indexes for better query performance
CREATE INDEX idx_question_quiz_id ON question(quiz_id);
CREATE INDEX idx_quiz_instance_quiz_id ON quiz_instance(quiz_id);
CREATE INDEX idx_quiz_instance_uuid ON quiz_instance(uuid);
CREATE INDEX idx_team_answer_question_id ON team_answer(question_id);
CREATE INDEX idx_team_answer_quiz_instance_id ON team_answer(quiz_instance_id);

-- Create trigger function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create triggers for updated_at
CREATE TRIGGER update_quiz_updated_at BEFORE UPDATE ON quiz
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_question_updated_at BEFORE UPDATE ON question
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
