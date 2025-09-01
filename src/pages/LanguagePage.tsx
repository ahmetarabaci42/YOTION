import React, { useState, useEffect } from 'react';
import { languageApi, vocabularyApi, flashcardApi, Language, Vocabulary, Flashcard } from '../api/tauri';

interface FlashcardWithVocab {
  flashcard: Flashcard;
  vocabulary: Vocabulary;
}

export default function LanguagePage() {
  const [languages, setLanguages] = useState<Language[]>([]);
  const [selectedLanguage, setSelectedLanguage] = useState<Language | null>(null);
  const [vocabulary, setVocabulary] = useState<Vocabulary[]>([]);
  const [dueFlashcards, setDueFlashcards] = useState<FlashcardWithVocab[]>([]);
  const [currentFlashcard, setCurrentFlashcard] = useState<FlashcardWithVocab | null>(null);
  const [showAnswer, setShowAnswer] = useState(false);
  const [activeTab, setActiveTab] = useState<'languages' | 'vocabulary' | 'flashcards'>('languages');
  
  // Form states
  const [showLanguageForm, setShowLanguageForm] = useState(false);
  const [showVocabForm, setShowVocabForm] = useState(false);
  const [languageForm, setLanguageForm] = useState({ name: '', code: '', flag_emoji: '' });
  const [vocabForm, setVocabForm] = useState({
    word: '', translation: '', pronunciation: '', example_sentence: '', difficulty_level: 1
  });

  useEffect(() => {
    loadLanguages();
    loadDueFlashcards();
  }, []);

  useEffect(() => {
    if (selectedLanguage) {
      loadVocabulary(selectedLanguage.id);
    }
  }, [selectedLanguage]);

  const loadLanguages = async () => {
    try {
      const langs = await languageApi.getAll();
      setLanguages(langs);
      if (langs.length > 0 && !selectedLanguage) {
        setSelectedLanguage(langs[0]);
      }
    } catch (error) {
      console.error('Failed to load languages:', error);
    }
  };

  const loadVocabulary = async (languageId: number) => {
    try {
      const vocab = await vocabularyApi.getByLanguage(languageId);
      setVocabulary(vocab);
    } catch (error) {
      console.error('Failed to load vocabulary:', error);
    }
  };

  const loadDueFlashcards = async () => {
    try {
      const flashcards = await flashcardApi.getDue(20);
      const formatted = flashcards.map(([flashcard, vocabulary]) => ({ flashcard, vocabulary }));
      setDueFlashcards(formatted);
      if (formatted.length > 0 && !currentFlashcard) {
        setCurrentFlashcard(formatted[0]);
      }
    } catch (error) {
      console.error('Failed to load flashcards:', error);
    }
  };

  const handleCreateLanguage = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await languageApi.create(languageForm);
      setLanguageForm({ name: '', code: '', flag_emoji: '' });
      setShowLanguageForm(false);
      loadLanguages();
    } catch (error) {
      console.error('Failed to create language:', error);
    }
  };

  const handleCreateVocabulary = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!selectedLanguage) return;
    
    try {
      await vocabularyApi.create({
        language_id: selectedLanguage.id,
        ...vocabForm
      });
      setVocabForm({ word: '', translation: '', pronunciation: '', example_sentence: '', difficulty_level: 1 });
      setShowVocabForm(false);
      loadVocabulary(selectedLanguage.id);
      loadDueFlashcards(); // Refresh flashcards as new ones are created
    } catch (error) {
      console.error('Failed to create vocabulary:', error);
    }
  };

  const handleFlashcardReview = async (quality: number) => {
    if (!currentFlashcard) return;
    
    try {
      await flashcardApi.review({ flashcard_id: currentFlashcard.flashcard.id, quality });
      
      // Remove current flashcard and move to next
      const remaining = dueFlashcards.filter(fc => fc.flashcard.id !== currentFlashcard.flashcard.id);
      setDueFlashcards(remaining);
      setCurrentFlashcard(remaining.length > 0 ? remaining[0] : null);
      setShowAnswer(false);
    } catch (error) {
      console.error('Failed to review flashcard:', error);
    }
  };

  const renderLanguagesTab = () => (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h3 className="text-xl font-semibold">Languages</h3>
        <button
          onClick={() => setShowLanguageForm(true)}
          className="btn-primary"
        >
          Add Language
        </button>
      </div>

      {showLanguageForm && (
        <div className="card">
          <form onSubmit={handleCreateLanguage} className="space-y-4">
            <div>
              <label className="block text-sm font-medium mb-1">Language Name</label>
              <input
                type="text"
                value={languageForm.name}
                onChange={(e) => setLanguageForm({...languageForm, name: e.target.value})}
                className="input-field"
                required
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-1">Language Code</label>
              <input
                type="text"
                value={languageForm.code}
                onChange={(e) => setLanguageForm({...languageForm, code: e.target.value})}
                className="input-field"
                placeholder="e.g., en, es, fr"
                required
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-1">Flag Emoji</label>
              <input
                type="text"
                value={languageForm.flag_emoji}
                onChange={(e) => setLanguageForm({...languageForm, flag_emoji: e.target.value})}
                className="input-field"
                placeholder="🇺🇸"
                required
              />
            </div>
            <div className="flex gap-2">
              <button type="submit" className="btn-primary">Create</button>
              <button
                type="button"
                onClick={() => setShowLanguageForm(false)}
                className="btn-secondary"
              >
                Cancel
              </button>
            </div>
          </form>
        </div>
      )}

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {languages.map((lang) => (
          <div
            key={lang.id}
            className={`card cursor-pointer transition-colors ${
              selectedLanguage?.id === lang.id ? 'ring-2 ring-notion-accent' : ''
            }`}
            onClick={() => setSelectedLanguage(lang)}
          >
            <div className="flex items-center space-x-3">
              <span className="text-2xl">{lang.flag_emoji}</span>
              <div>
                <h4 className="font-medium">{lang.name}</h4>
                <p className="text-sm text-notion-muted">{lang.code}</p>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );

  const renderVocabularyTab = () => (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h3 className="text-xl font-semibold">
          Vocabulary {selectedLanguage && `- ${selectedLanguage.name}`}
        </h3>
        <button
          onClick={() => setShowVocabForm(true)}
          className="btn-primary"
          disabled={!selectedLanguage}
        >
          Add Word
        </button>
      </div>

      {showVocabForm && selectedLanguage && (
        <div className="card">
          <form onSubmit={handleCreateVocabulary} className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium mb-1">Word</label>
                <input
                  type="text"
                  value={vocabForm.word}
                  onChange={(e) => setVocabForm({...vocabForm, word: e.target.value})}
                  className="input-field"
                  required
                />
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">Translation</label>
                <input
                  type="text"
                  value={vocabForm.translation}
                  onChange={(e) => setVocabForm({...vocabForm, translation: e.target.value})}
                  className="input-field"
                  required
                />
              </div>
            </div>
            <div>
              <label className="block text-sm font-medium mb-1">Pronunciation (optional)</label>
              <input
                type="text"
                value={vocabForm.pronunciation}
                onChange={(e) => setVocabForm({...vocabForm, pronunciation: e.target.value})}
                className="input-field"
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-1">Example Sentence (optional)</label>
              <textarea
                value={vocabForm.example_sentence}
                onChange={(e) => setVocabForm({...vocabForm, example_sentence: e.target.value})}
                className="input-field"
                rows={2}
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-1">Difficulty Level</label>
              <select
                value={vocabForm.difficulty_level}
                onChange={(e) => setVocabForm({...vocabForm, difficulty_level: parseInt(e.target.value)})}
                className="input-field"
              >
                <option value={1}>Beginner</option>
                <option value={2}>Intermediate</option>
                <option value={3}>Advanced</option>
              </select>
            </div>
            <div className="flex gap-2">
              <button type="submit" className="btn-primary">Add Word</button>
              <button
                type="button"
                onClick={() => setShowVocabForm(false)}
                className="btn-secondary"
              >
                Cancel
              </button>
            </div>
          </form>
        </div>
      )}

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {vocabulary.map((word) => (
          <div key={word.id} className="card">
            <div className="space-y-2">
              <div className="flex justify-between items-start">
                <h4 className="font-medium text-lg">{word.word}</h4>
                <span className="text-xs bg-notion-hover px-2 py-1 rounded">
                  Level {word.difficulty_level}
                </span>
              </div>
              <p className="text-notion-muted">{word.translation}</p>
              {word.pronunciation && (
                <p className="text-sm text-notion-muted italic">/{word.pronunciation}/</p>
              )}
              {word.example_sentence && (
                <p className="text-sm bg-notion-hover p-2 rounded italic">
                  "{word.example_sentence}"
                </p>
              )}
            </div>
          </div>
        ))}
      </div>
    </div>
  );

  const renderFlashcardsTab = () => (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h3 className="text-xl font-semibold">Flashcards Review</h3>
        <span className="text-sm text-notion-muted">
          {dueFlashcards.length} cards due
        </span>
      </div>

      {currentFlashcard ? (
        <div className="max-w-md mx-auto">
          <div className="card text-center space-y-6">
            <div className="space-y-4">
              <h4 className="text-2xl font-bold">{currentFlashcard.vocabulary.word}</h4>
              {currentFlashcard.vocabulary.pronunciation && (
                <p className="text-notion-muted italic">/{currentFlashcard.vocabulary.pronunciation}/</p>
              )}
            </div>

            {showAnswer ? (
              <div className="space-y-4">
                <div className="border-t pt-4">
                  <p className="text-xl text-notion-accent font-medium">
                    {currentFlashcard.vocabulary.translation}
                  </p>
                  {currentFlashcard.vocabulary.example_sentence && (
                    <p className="text-sm text-notion-muted mt-2 italic">
                      "{currentFlashcard.vocabulary.example_sentence}"
                    </p>
                  )}
                </div>
                
                <div className="space-y-2">
                  <p className="text-sm font-medium">How well did you know this?</p>
                  <div className="grid grid-cols-3 gap-2">
                    <button
                      onClick={() => handleFlashcardReview(1)}
                      className="px-3 py-2 bg-red-100 text-red-700 rounded hover:bg-red-200 text-sm"
                    >
                      Hard
                    </button>
                    <button
                      onClick={() => handleFlashcardReview(3)}
                      className="px-3 py-2 bg-yellow-100 text-yellow-700 rounded hover:bg-yellow-200 text-sm"
                    >
                      Good
                    </button>
                    <button
                      onClick={() => handleFlashcardReview(5)}
                      className="px-3 py-2 bg-green-100 text-green-700 rounded hover:bg-green-200 text-sm"
                    >
                      Easy
                    </button>
                  </div>
                </div>
              </div>
            ) : (
              <button
                onClick={() => setShowAnswer(true)}
                className="btn-primary w-full"
              >
                Show Answer
              </button>
            )}
          </div>
        </div>
      ) : (
        <div className="text-center py-12">
          <p className="text-notion-muted">No flashcards due for review!</p>
          <p className="text-sm text-notion-muted mt-2">Great job! Come back later for more practice.</p>
        </div>
      )}
    </div>
  );

  return (
    <div className="space-y-6">
      <div className="flex space-x-1 bg-notion-hover p-1 rounded-lg w-fit">
        {[
          { key: 'languages', label: 'Languages', icon: '🌍' },
          { key: 'vocabulary', label: 'Vocabulary', icon: '📚' },
          { key: 'flashcards', label: 'Flashcards', icon: '🎯' },
        ].map((tab) => (
          <button
            key={tab.key}
            onClick={() => setActiveTab(tab.key as any)}
            className={`px-4 py-2 rounded-md text-sm font-medium transition-colors ${
              activeTab === tab.key
                ? 'bg-white text-notion-text shadow-sm'
                : 'text-notion-muted hover:text-notion-text'
            }`}
          >
            <span className="mr-2">{tab.icon}</span>
            {tab.label}
          </button>
        ))}
      </div>

      {activeTab === 'languages' && renderLanguagesTab()}
      {activeTab === 'vocabulary' && renderVocabularyTab()}
      {activeTab === 'flashcards' && renderFlashcardsTab()}
    </div>
  );
}
