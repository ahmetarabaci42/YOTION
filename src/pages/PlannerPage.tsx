import React, { useState, useEffect } from 'react';
import { 
  Event, 
  Note, 
  createEvent, 
  getEventsByDate, 
  createNote, 
  getNotesByDate 
} from '../api/tauri';

const PlannerPage: React.FC = () => {
  const [activeTab, setActiveTab] = useState<'calendar' | 'notes'>('calendar');
  const [selectedDate, setSelectedDate] = useState<string>(new Date().toISOString().split('T')[0]);
  const [events, setEvents] = useState<Event[]>([]);
  const [notes, setNotes] = useState<Note[]>([]);
  
  // Form states
  const [showEventForm, setShowEventForm] = useState(false);
  const [showNoteForm, setShowNoteForm] = useState(false);
  const [eventForm, setEventForm] = useState({
    title: '',
    description: '',
    start_time: '',
    end_time: '',
    event_type: 'meeting',
    priority: 'medium'
  });
  const [noteForm, setNoteForm] = useState({
    title: '',
    content: '',
    tags: ''
  });

  useEffect(() => {
    loadEventsForDate(selectedDate);
    loadNotesForDate(selectedDate);
  }, [selectedDate]);

  const loadEventsForDate = async (date: string) => {
    try {
      const dayEvents = await getEventsByDate(date);
      setEvents(dayEvents);
    } catch (error) {
      console.error('Failed to load events:', error);
    }
  };

  const loadNotesForDate = async (date: string) => {
    try {
      const dayNotes = await getNotesByDate(date);
      setNotes(dayNotes);
    } catch (error) {
      console.error('Failed to load notes:', error);
    }
  };

  const handleCreateEvent = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await createEvent({
        event_date: selectedDate,
        ...eventForm
      });
      setEventForm({
        title: '',
        description: '',
        start_time: '',
        end_time: '',
        event_type: 'meeting',
        priority: 'medium'
      });
      setShowEventForm(false);
      loadEventsForDate(selectedDate);
    } catch (error) {
      console.error('Failed to create event:', error);
    }
  };

  const handleCreateNote = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await createNote({
        note_date: selectedDate,
        ...noteForm
      });
      setNoteForm({
        title: '',
        content: '',
        tags: ''
      });
      setShowNoteForm(false);
      loadNotesForDate(selectedDate);
    } catch (error) {
      console.error('Failed to create note:', error);
    }
  };

  const getEventTypeIcon = (type: string) => {
    switch (type) {
      case 'meeting': return '🤝';
      case 'task': return '✅';
      case 'reminder': return '⏰';
      case 'appointment': return '📅';
      default: return '📌';
    }
  };

  const getPriorityColor = (priority: string) => {
    switch (priority) {
      case 'high': return 'bg-red-100 text-red-800 border-red-200';
      case 'medium': return 'bg-yellow-100 text-yellow-800 border-yellow-200';
      case 'low': return 'bg-green-100 text-green-800 border-green-200';
      default: return 'bg-gray-100 text-gray-800 border-gray-200';
    }
  };

  const formatTime = (time: string) => {
    if (!time) return '';
    return new Date(`2000-01-01T${time}`).toLocaleTimeString('en-US', {
      hour: 'numeric',
      minute: '2-digit',
      hour12: true
    });
  };

  const generateCalendarDays = () => {
    const today = new Date();
    const currentMonth = today.getMonth();
    const currentYear = today.getFullYear();
    
    const firstDay = new Date(currentYear, currentMonth, 1);
    const startDate = new Date(firstDay);
    startDate.setDate(startDate.getDate() - firstDay.getDay());
    
    const days = [];
    for (let i = 0; i < 42; i++) {
      const date = new Date(startDate);
      date.setDate(startDate.getDate() + i);
      days.push(date);
    }
    
    return days;
  };

  const isToday = (date: Date) => {
    const today = new Date();
    return date.toDateString() === today.toDateString();
  };

  const isSelectedDate = (date: Date) => {
    return date.toISOString().split('T')[0] === selectedDate;
  };

  const isCurrentMonth = (date: Date) => {
    const today = new Date();
    return date.getMonth() === today.getMonth() && date.getFullYear() === today.getFullYear();
  };

  return (
    <div className="h-full flex flex-col">
      {/* Header */}
      <div className="flex items-center justify-between p-6 border-b border-gray-200">
        <div>
          <h1 className="text-2xl font-semibold text-gray-900">Planner</h1>
          <p className="text-sm text-gray-600 mt-1">
            Selected: {new Date(selectedDate).toLocaleDateString('en-US', { 
              weekday: 'long', 
              year: 'numeric', 
              month: 'long', 
              day: 'numeric' 
            })}
          </p>
        </div>
        <div className="flex space-x-2">
          <button
            onClick={() => setShowEventForm(true)}
            className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
          >
            New Event
          </button>
          <button
            onClick={() => setShowNoteForm(true)}
            className="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors"
          >
            New Note
          </button>
        </div>
      </div>

      {/* Tabs */}
      <div className="flex border-b border-gray-200">
        <button
          onClick={() => setActiveTab('calendar')}
          className={`px-6 py-3 font-medium ${
            activeTab === 'calendar'
              ? 'text-blue-600 border-b-2 border-blue-600'
              : 'text-gray-500 hover:text-gray-700'
          }`}
        >
          Calendar
        </button>
        <button
          onClick={() => setActiveTab('notes')}
          className={`px-6 py-3 font-medium ${
            activeTab === 'notes'
              ? 'text-blue-600 border-b-2 border-blue-600'
              : 'text-gray-500 hover:text-gray-700'
          }`}
        >
          Daily Notes
        </button>
      </div>

      {/* Content */}
      <div className="flex-1 p-6 overflow-auto">
        {activeTab === 'calendar' && (
          <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
            {/* Calendar */}
            <div className="lg:col-span-2">
              <div className="bg-white rounded-lg border border-gray-200 p-4">
                <div className="flex items-center justify-between mb-4">
                  <h3 className="text-lg font-semibold">
                    {new Date().toLocaleDateString('en-US', { month: 'long', year: 'numeric' })}
                  </h3>
                </div>
                
                <div className="grid grid-cols-7 gap-1 mb-2">
                  {['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'].map(day => (
                    <div key={day} className="p-2 text-center text-sm font-medium text-gray-500">
                      {day}
                    </div>
                  ))}
                </div>
                
                <div className="grid grid-cols-7 gap-1">
                  {generateCalendarDays().map((date, index) => (
                    <button
                      key={index}
                      onClick={() => setSelectedDate(date.toISOString().split('T')[0])}
                      className={`p-2 text-sm rounded transition-colors ${
                        isSelectedDate(date)
                          ? 'bg-blue-600 text-white'
                          : isToday(date)
                          ? 'bg-blue-100 text-blue-800'
                          : isCurrentMonth(date)
                          ? 'text-gray-900 hover:bg-gray-100'
                          : 'text-gray-400 hover:bg-gray-50'
                      }`}
                    >
                      {date.getDate()}
                    </button>
                  ))}
                </div>
              </div>
            </div>

            {/* Events for selected date */}
            <div>
              <h3 className="text-lg font-semibold mb-4">Events</h3>
              <div className="space-y-3">
                {events.map((event) => (
                  <div key={event.id} className={`p-3 rounded-lg border ${getPriorityColor(event.priority)}`}>
                    <div className="flex items-start space-x-2">
                      <span className="text-lg">{getEventTypeIcon(event.event_type)}</span>
                      <div className="flex-1">
                        <h4 className="font-medium">{event.title}</h4>
                        {event.description && (
                          <p className="text-sm opacity-80 mt-1">{event.description}</p>
                        )}
                        {(event.start_time || event.end_time) && (
                          <div className="text-xs mt-2">
                            {event.start_time && formatTime(event.start_time)}
                            {event.start_time && event.end_time && ' - '}
                            {event.end_time && formatTime(event.end_time)}
                          </div>
                        )}
                      </div>
                    </div>
                  </div>
                ))}
                {events.length === 0 && (
                  <p className="text-gray-500 text-center py-8">No events for this date</p>
                )}
              </div>
            </div>
          </div>
        )}

        {activeTab === 'notes' && (
          <div className="max-w-4xl mx-auto">
            <h2 className="text-xl font-semibold text-gray-900 mb-6">
              📝 Notes for {new Date(selectedDate).toLocaleDateString()}
            </h2>
            <div className="space-y-4">
              {notes.map((note) => (
                <div key={note.id} className="bg-white rounded-lg border border-gray-200 p-4">
                  <div className="flex items-start justify-between mb-3">
                    <h3 className="font-semibold text-gray-900">{note.title}</h3>
                    {note.tags && (
                      <div className="flex flex-wrap gap-1">
                        {note.tags.split(',').map((tag, index) => (
                          <span key={index} className="px-2 py-1 bg-gray-100 text-gray-700 text-xs rounded">
                            {tag.trim()}
                          </span>
                        ))}
                      </div>
                    )}
                  </div>
                  <div className="prose prose-sm max-w-none">
                    <p className="text-gray-700 whitespace-pre-wrap">{note.content}</p>
                  </div>
                  <div className="text-xs text-gray-500 mt-3">
                    Created: {new Date(note.created_at).toLocaleString()}
                  </div>
                </div>
              ))}
              {notes.length === 0 && (
                <p className="text-gray-500 text-center py-8">No notes for this date</p>
              )}
            </div>
          </div>
        )}
      </div>

      {/* Create Event Modal */}
      {showEventForm && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 w-full max-w-md">
            <h2 className="text-xl font-semibold mb-4">Create Event</h2>
            <form onSubmit={handleCreateEvent} className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Title</label>
                <input
                  type="text"
                  value={eventForm.title}
                  onChange={(e) => setEventForm({ ...eventForm, title: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  required
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Description</label>
                <textarea
                  value={eventForm.description}
                  onChange={(e) => setEventForm({ ...eventForm, description: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  rows={3}
                />
              </div>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Start Time</label>
                  <input
                    type="time"
                    value={eventForm.start_time}
                    onChange={(e) => setEventForm({ ...eventForm, start_time: e.target.value })}
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">End Time</label>
                  <input
                    type="time"
                    value={eventForm.end_time}
                    onChange={(e) => setEventForm({ ...eventForm, end_time: e.target.value })}
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  />
                </div>
              </div>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Type</label>
                  <select
                    value={eventForm.event_type}
                    onChange={(e) => setEventForm({ ...eventForm, event_type: e.target.value })}
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  >
                    <option value="meeting">Meeting</option>
                    <option value="task">Task</option>
                    <option value="reminder">Reminder</option>
                    <option value="appointment">Appointment</option>
                    <option value="event">Event</option>
                  </select>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Priority</label>
                  <select
                    value={eventForm.priority}
                    onChange={(e) => setEventForm({ ...eventForm, priority: e.target.value })}
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  >
                    <option value="low">Low</option>
                    <option value="medium">Medium</option>
                    <option value="high">High</option>
                  </select>
                </div>
              </div>
              <div className="flex space-x-3">
                <button
                  type="submit"
                  className="flex-1 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
                >
                  Create
                </button>
                <button
                  type="button"
                  onClick={() => setShowEventForm(false)}
                  className="flex-1 px-4 py-2 bg-gray-300 text-gray-700 rounded-lg hover:bg-gray-400 transition-colors"
                >
                  Cancel
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      {/* Create Note Modal */}
      {showNoteForm && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 w-full max-w-2xl">
            <h2 className="text-xl font-semibold mb-4">Create Note</h2>
            <form onSubmit={handleCreateNote} className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Title</label>
                <input
                  type="text"
                  value={noteForm.title}
                  onChange={(e) => setNoteForm({ ...noteForm, title: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  required
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Content</label>
                <textarea
                  value={noteForm.content}
                  onChange={(e) => setNoteForm({ ...noteForm, content: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  rows={8}
                  required
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Tags</label>
                <input
                  type="text"
                  value={noteForm.tags}
                  onChange={(e) => setNoteForm({ ...noteForm, tags: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  placeholder="work, personal, ideas (comma separated)"
                />
              </div>
              <div className="flex space-x-3">
                <button
                  type="submit"
                  className="flex-1 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
                >
                  Create
                </button>
                <button
                  type="button"
                  onClick={() => setShowNoteForm(false)}
                  className="flex-1 px-4 py-2 bg-gray-300 text-gray-700 rounded-lg hover:bg-gray-400 transition-colors"
                >
                  Cancel
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
};

export default PlannerPage;
