-- Languages table
CREATE TABLE IF NOT EXISTS languages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    code TEXT NOT NULL UNIQUE,
    flag_emoji TEXT NOT NULL,
    created_at TEXT NOT NULL
);

-- Vocabulary table
CREATE TABLE IF NOT EXISTS vocabulary (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    language_id INTEGER NOT NULL,
    word TEXT NOT NULL,
    translation TEXT NOT NULL,
    pronunciation TEXT,
    example_sentence TEXT,
    difficulty_level INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    FOREIGN KEY (language_id) REFERENCES languages(id) ON DELETE CASCADE
);

-- Flashcards table for spaced repetition
CREATE TABLE IF NOT EXISTS flashcards (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    vocabulary_id INTEGER NOT NULL,
    ease_factor REAL NOT NULL DEFAULT 2.5,
    interval_days INTEGER NOT NULL DEFAULT 1,
    repetitions INTEGER NOT NULL DEFAULT 0,
    next_review TEXT NOT NULL,
    last_reviewed TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (vocabulary_id) REFERENCES vocabulary(id) ON DELETE CASCADE
);

-- Tech spaces (programming languages, frameworks, etc.)
CREATE TABLE IF NOT EXISTS tech_spaces (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    icon TEXT NOT NULL,
    created_at TEXT NOT NULL
);

-- Folder structure for tech notes
CREATE TABLE IF NOT EXISTS tech_folders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    tech_space_id INTEGER NOT NULL,
    parent_id INTEGER,
    name TEXT NOT NULL,
    path TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (tech_space_id) REFERENCES tech_spaces(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_id) REFERENCES tech_folders(id) ON DELETE CASCADE
);

-- Code snippets
CREATE TABLE IF NOT EXISTS code_snippets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    folder_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    code TEXT NOT NULL,
    language TEXT NOT NULL,
    tags TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (folder_id) REFERENCES tech_folders(id) ON DELETE CASCADE
);

-- Projects
CREATE TABLE IF NOT EXISTS projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'planning',
    priority TEXT NOT NULL DEFAULT 'medium',
    requirements TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Daily plans
CREATE TABLE IF NOT EXISTS daily_plans (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date TEXT NOT NULL UNIQUE,
    tasks TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Weekly plans
CREATE TABLE IF NOT EXISTS weekly_plans (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    week_start TEXT NOT NULL UNIQUE,
    goals TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Create FTS5 virtual tables for full-text search
CREATE VIRTUAL TABLE IF NOT EXISTS vocabulary_fts USING fts5(
    word, translation, example_sentence,
    content='vocabulary',
    content_rowid='id'
);

CREATE VIRTUAL TABLE IF NOT EXISTS code_snippets_fts USING fts5(
    title, description, code, tags,
    content='code_snippets',
    content_rowid='id'
);

-- Triggers to keep FTS tables in sync
CREATE TRIGGER IF NOT EXISTS vocabulary_fts_insert AFTER INSERT ON vocabulary BEGIN
    INSERT INTO vocabulary_fts(rowid, word, translation, example_sentence)
    VALUES (new.id, new.word, new.translation, new.example_sentence);
END;

CREATE TRIGGER IF NOT EXISTS vocabulary_fts_delete AFTER DELETE ON vocabulary BEGIN
    DELETE FROM vocabulary_fts WHERE rowid = old.id;
END;

CREATE TRIGGER IF NOT EXISTS vocabulary_fts_update AFTER UPDATE ON vocabulary BEGIN
    DELETE FROM vocabulary_fts WHERE rowid = old.id;
    INSERT INTO vocabulary_fts(rowid, word, translation, example_sentence)
    VALUES (new.id, new.word, new.translation, new.example_sentence);
END;

CREATE TRIGGER IF NOT EXISTS code_snippets_fts_insert AFTER INSERT ON code_snippets BEGIN
    INSERT INTO code_snippets_fts(rowid, title, description, code, tags)
    VALUES (new.id, new.title, new.description, new.code, new.tags);
END;

CREATE TRIGGER IF NOT EXISTS code_snippets_fts_delete AFTER DELETE ON code_snippets BEGIN
    DELETE FROM code_snippets_fts WHERE rowid = old.id;
END;

CREATE TRIGGER IF NOT EXISTS code_snippets_fts_update AFTER UPDATE ON code_snippets BEGIN
    DELETE FROM code_snippets_fts WHERE rowid = old.id;
    INSERT INTO code_snippets_fts(rowid, title, description, code, tags)
    VALUES (new.id, new.title, new.description, new.code, new.tags);
END;
