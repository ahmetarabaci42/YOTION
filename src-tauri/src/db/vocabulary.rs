use crate::models::*;
use crate::db::Database;
use crate::validation::*;
use anyhow::Result;
use chrono::Utc;

impl Database {
    pub fn create_vocabulary(&self, req: CreateVocabularyRequest) -> Result<Vocabulary> {
validate_not_empty(&req.word, "Word")?;
validate_string_length(&req.word, "Word", 1, 200)?;
validate_not_empty(&req.translation, "Translation")?;
validate_string_length(&req.translation, "Translation", 1, 500)?;
validate_difficulty_level(req.difficulty_level)?;

        let word = sanitize_string(req.word);
        let translation = sanitize_string(req.translation);
        let pronunciation = sanitize_optional_string(req.pronunciation);
        let example_sentence = sanitize_optional_string(req.example_sentence);

        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO vocabulary (language_id, word, translation, pronunciation, example_sentence, difficulty_level, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (&req.language_id, &word, &translation, &pronunciation, &example_sentence, &req.difficulty_level, &now),
        )?;

        let vocab_id = conn.last_insert_rowid();

        conn.execute(
            "INSERT INTO flashcards (vocabulary_id, ease_factor, interval_days, repetitions, next_review, created_at)
             VALUES (?1, 2.5, 1, 0, ?2, ?3)",
            (vocab_id, &now, &now),
        )?;

        Ok(Vocabulary {
            id: vocab_id,
            language_id: req.language_id,
            word,
            translation,
            pronunciation,
            example_sentence,
            difficulty_level: req.difficulty_level,
            created_at: now,
        })
    }

    pub fn get_vocabulary_by_language(&self, language_id: i64) -> Result<Vec<Vocabulary>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, language_id, word, translation, pronunciation, example_sentence, difficulty_level, created_at
             FROM vocabulary WHERE language_id = ?1 ORDER BY created_at DESC"
        )?;

        let vocab_iter = stmt.query_map([language_id], |row| {
            Ok(Vocabulary {
                id: row.get(0)?,
                language_id: row.get(1)?,
                word: row.get(2)?,
                translation: row.get(3)?,
                pronunciation: row.get(4)?,
                example_sentence: row.get(5)?,
                difficulty_level: row.get(6)?,
                created_at: row.get(7)?,
            })
        })?;

        let mut vocabulary = Vec::new();
        for vocab in vocab_iter {
            vocabulary.push(vocab?);
        }

        Ok(vocabulary)
    }

    pub fn get_due_flashcards(&self, limit: i64) -> Result<Vec<(Flashcard, Vocabulary)>> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        let mut stmt = conn.prepare(
            "SELECT f.id, f.vocabulary_id, f.ease_factor, f.interval_days, f.repetitions,
                    f.next_review, f.last_reviewed, f.created_at,
                    v.language_id, v.word, v.translation, v.pronunciation, v.example_sentence, v.difficulty_level, v.created_at as vocab_created_at
             FROM flashcards f
             JOIN vocabulary v ON f.vocabulary_id = v.id
             WHERE f.next_review <= ?1
             ORDER BY f.next_review ASC
             LIMIT ?2"
        )?;

        let flashcard_iter = stmt.query_map([&now, &limit.to_string()], |row| {
            let flashcard = Flashcard {
                id: row.get(0)?,
                vocabulary_id: row.get(1)?,
                ease_factor: row.get(2)?,
                interval_days: row.get(3)?,
                repetitions: row.get(4)?,
                next_review: row.get(5)?,
                last_reviewed: row.get(6)?,
                created_at: row.get(7)?,
            };

            let vocabulary = Vocabulary {
                id: row.get(1)?,
                language_id: row.get(8)?,
                word: row.get(9)?,
                translation: row.get(10)?,
                pronunciation: row.get(11)?,
                example_sentence: row.get(12)?,
                difficulty_level: row.get(13)?,
                created_at: row.get(14)?,
            };

            Ok((flashcard, vocabulary))
        })?;

        let mut result = Vec::new();
        for item in flashcard_iter {
            result.push(item?);
        }

        Ok(result)
    }

    pub fn review_flashcard(&self, req: FlashcardReviewRequest) -> Result<()> {
validate_quality_rating(req.quality)?;

        let conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT ease_factor, interval_days, repetitions FROM flashcards WHERE id = ?1"
        )?;
        let (mut ease_factor, mut interval, mut repetitions): (f64, i32, i32) = stmt.query_row(
            [req.flashcard_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        )?;

        let quality = req.quality.max(0).min(5);

        if quality >= 3 {
            if repetitions == 0 {
                interval = 1;
            } else if repetitions == 1 {
                interval = 6;
            } else {
                interval = (interval as f64 * ease_factor).round() as i32;
            }
            repetitions += 1;
        } else {
            repetitions = 0;
            interval = 1;
        }

        ease_factor = ease_factor + (0.1 - (5.0 - quality as f64) * (0.08 + (5.0 - quality as f64) * 0.02));
        ease_factor = ease_factor.max(1.3);

        let next_review = (Utc::now() + chrono::Duration::days(interval as i64)).to_rfc3339();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "UPDATE flashcards SET ease_factor = ?1, interval_days = ?2, repetitions = ?3,
             next_review = ?4, last_reviewed = ?5 WHERE id = ?6",
            (ease_factor, interval, repetitions, &next_review, &now, req.flashcard_id),
        )?;

        Ok(())
    }
}
