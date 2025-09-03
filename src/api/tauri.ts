import { invoke } from '@tauri-apps/api/core';

// Types
export interface Language {
  id: number;
  name: string;
  code: string;
  flag_emoji: string;
  created_at: string;
}

export interface Vocabulary {
  id: number;
  language_id: number;
  word: string;
  translation: string;
  pronunciation?: string;
  example_sentence?: string;
  difficulty_level: number;
  created_at: string;
}

export interface Flashcard {
  id: number;
  vocabulary_id: number;
  ease_factor: number;
  interval_days: number;
  repetitions: number;
  next_review: string;
  last_reviewed?: string;
  created_at: string;
}

export interface TechSpace {
  id: number;
  name: string;
  description?: string;
  icon: string;
  created_at: string;
}

export interface CodeSnippet {
  id: number;
  tech_space_id: number;
  title: string;
  description?: string;
  code: string;
  language: string;
  tags?: string;
  created_at: string;
  updated_at: string;
}

// Language API
export const languageApi = {
  create: (req: { name: string; code: string; flag_emoji: string }): Promise<Language> =>
    invoke('create_language', { req }),
  
  getAll: (): Promise<Language[]> =>
    invoke('get_languages'),
  
  delete: (id: number): Promise<void> =>
    invoke('delete_language', { id }),
};

// Vocabulary API
export const vocabularyApi = {
  create: (req: {
    language_id: number;
    word: string;
    translation: string;
    pronunciation?: string;
    example_sentence?: string;
    difficulty_level: number;
  }): Promise<Vocabulary> =>
    invoke('create_vocabulary', { req }),
  
  getByLanguage: (languageId: number): Promise<Vocabulary[]> =>
    invoke('get_vocabulary_by_language', { languageId }),
  
  search: (query: string, limit?: number): Promise<Vocabulary[]> =>
    invoke('search_vocabulary', { query, limit }),
};

// Flashcard API
export const flashcardApi = {
  getDue: (limit?: number): Promise<[Flashcard, Vocabulary][]> =>
    invoke('get_due_flashcards', { limit }),
  
  review: (req: { flashcard_id: number; quality: number }): Promise<void> =>
    invoke('review_flashcard', { req }),
};

// Tech Space API
export const createTechSpace = async (data: {
  name: string;
  description?: string;
  icon: string;
}): Promise<TechSpace> => {
  return await invoke('create_tech_space', { req: data });
};

export const getTechSpaces = async (): Promise<TechSpace[]> => {
  return await invoke('get_tech_spaces');
};

// Code Snippet API
export const createCodeSnippet = async (data: {
  tech_space_id: number;
  title: string;
  description?: string;
  code: string;
  language: string;
  tags?: string;
}): Promise<CodeSnippet> => {
  return await invoke('create_code_snippet', { req: data });
};

export const getCodeSnippetsByTechSpace = async (techSpaceId: number): Promise<CodeSnippet[]> => {
  return await invoke('get_code_snippets_by_tech_space', { techSpaceId });
};

export const searchCodeSnippets = async (query: string, limit?: number): Promise<CodeSnippet[]> => {
  return await invoke('search_code_snippets', { query, limit });
};

// Project interfaces
export interface Project {
  id: number;
  name: string;
  description?: string;
  status: string;
  priority: string;
  start_date?: string;
  end_date?: string;
  created_at: string;
  updated_at: string;
}

export interface Task {
  id: number;
  project_id: number;
  title: string;
  description?: string;
  status: string;
  priority: string;
  due_date?: string;
  completed_at?: string;
  created_at: string;
  updated_at: string;
}

// Project API
export const createProject = async (data: {
  name: string;
  description?: string;
  status: string;
  priority: string;
  start_date?: string;
  end_date?: string;
}): Promise<Project> => {
  return await invoke('create_project', { req: data });
};

export const getProjects = async (): Promise<Project[]> => {
  return await invoke('get_projects');
};

// Task API
export const createTask = async (data: {
  project_id: number;
  title: string;
  description?: string;
  status: string;
  priority: string;
  due_date?: string;
}): Promise<Task> => {
  return await invoke('create_task', { req: data });
};

export const getTasksByProject = async (projectId: number): Promise<Task[]> => {
  return await invoke('get_tasks_by_project', { projectId });
};

export const updateTaskStatus = async (taskId: number, status: string): Promise<void> => {
  return await invoke('update_task_status', { taskId, status });
};

// Planner interfaces
export interface Event {
  id: number;
  title: string;
  description?: string;
  event_date: string;
  start_time?: string;
  end_time?: string;
  event_type: string;
  priority: string;
  created_at: string;
  updated_at: string;
}

export interface Note {
  id: number;
  title: string;
  content: string;
  note_date: string;
  tags?: string;
  created_at: string;
  updated_at: string;
}

// Event API
export const createEvent = async (data: {
  title: string;
  description?: string;
  event_date: string;
  start_time?: string;
  end_time?: string;
  event_type: string;
  priority: string;
}): Promise<Event> => {
  return await invoke('create_event', { req: data });
};

export const getEventsByDate = async (date: string): Promise<Event[]> => {
  return await invoke('get_events_by_date', { date });
};

// Note API
export const createNote = async (data: {
  title: string;
  content: string;
  note_date: string;
  tags?: string;
}): Promise<Note> => {
  return await invoke('create_note', { req: data });
};

export const getNotesByDate = async (date: string): Promise<Note[]> => {
  return await invoke('get_notes_by_date', { date });
};
