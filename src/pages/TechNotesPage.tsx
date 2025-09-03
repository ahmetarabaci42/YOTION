import React, { useState, useEffect } from 'react';
import { 
  TechSpace, 
  CodeSnippet, 
  createTechSpace, 
  getTechSpaces, 
  createCodeSnippet, 
  getCodeSnippetsByTechSpace,
  searchCodeSnippets 
} from '../api/tauri';

const TechNotesPage: React.FC = () => {
  const [activeTab, setActiveTab] = useState<'spaces' | 'snippets' | 'search'>('spaces');
  const [techSpaces, setTechSpaces] = useState<TechSpace[]>([]);
  const [selectedSpace, setSelectedSpace] = useState<TechSpace | null>(null);
  const [snippets, setSnippets] = useState<CodeSnippet[]>([]);
  const [searchResults, setSearchResults] = useState<CodeSnippet[]>([]);
  const [searchQuery, setSearchQuery] = useState('');
  
  // Form states
  const [showSpaceForm, setShowSpaceForm] = useState(false);
  const [showSnippetForm, setShowSnippetForm] = useState(false);
  const [spaceForm, setSpaceForm] = useState({ name: '', description: '', icon: '💻' });
  const [snippetForm, setSnippetForm] = useState({
    title: '',
    description: '',
    code: '',
    language: 'javascript',
    tags: ''
  });

  useEffect(() => {
    loadTechSpaces();
  }, []);

  useEffect(() => {
    if (selectedSpace) {
      loadSnippets(selectedSpace.id);
    }
  }, [selectedSpace]);

  const loadTechSpaces = async () => {
    try {
      const spaces = await getTechSpaces();
      setTechSpaces(spaces);
      if (spaces.length > 0 && !selectedSpace) {
        setSelectedSpace(spaces[0]);
      }
    } catch (error) {
      console.error('Failed to load tech spaces:', error);
    }
  };

  const loadSnippets = async (spaceId: number) => {
    try {
      const spaceSnippets = await getCodeSnippetsByTechSpace(spaceId);
      setSnippets(spaceSnippets);
    } catch (error) {
      console.error('Failed to load snippets:', error);
    }
  };

  const handleCreateSpace = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await createTechSpace(spaceForm);
      setSpaceForm({ name: '', description: '', icon: '💻' });
      setShowSpaceForm(false);
      loadTechSpaces();
    } catch (error) {
      console.error('Failed to create tech space:', error);
    }
  };

  const handleCreateSnippet = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!selectedSpace) return;
    
    try {
      await createCodeSnippet({
        tech_space_id: selectedSpace.id,
        ...snippetForm
      });
      setSnippetForm({
        title: '',
        description: '',
        code: '',
        language: 'javascript',
        tags: ''
      });
      setShowSnippetForm(false);
      loadSnippets(selectedSpace.id);
    } catch (error) {
      console.error('Failed to create snippet:', error);
    }
  };

  const handleSearch = async () => {
    if (!searchQuery.trim()) return;
    
    try {
      const results = await searchCodeSnippets(searchQuery, 20);
      setSearchResults(results);
    } catch (error) {
      console.error('Failed to search snippets:', error);
    }
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  return (
    <div className="h-full flex flex-col">
      {/* Header */}
      <div className="flex items-center justify-between p-6 border-b border-gray-200">
        <h1 className="text-2xl font-semibold text-gray-900">Tech Notes</h1>
        <div className="flex space-x-2">
          <button
            onClick={() => setShowSpaceForm(true)}
            className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
          >
            New Space
          </button>
          {selectedSpace && (
            <button
              onClick={() => setShowSnippetForm(true)}
              className="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors"
            >
              New Snippet
            </button>
          )}
        </div>
      </div>

      {/* Tabs */}
      <div className="flex border-b border-gray-200">
        <button
          onClick={() => setActiveTab('spaces')}
          className={`px-6 py-3 font-medium ${
            activeTab === 'spaces'
              ? 'text-blue-600 border-b-2 border-blue-600'
              : 'text-gray-500 hover:text-gray-700'
          }`}
        >
          Spaces
        </button>
        <button
          onClick={() => setActiveTab('snippets')}
          className={`px-6 py-3 font-medium ${
            activeTab === 'snippets'
              ? 'text-blue-600 border-b-2 border-blue-600'
              : 'text-gray-500 hover:text-gray-700'
          }`}
        >
          Snippets
        </button>
        <button
          onClick={() => setActiveTab('search')}
          className={`px-6 py-3 font-medium ${
            activeTab === 'search'
              ? 'text-blue-600 border-b-2 border-blue-600'
              : 'text-gray-500 hover:text-gray-700'
          }`}
        >
          Search
        </button>
      </div>

      {/* Content */}
      <div className="flex-1 p-6 overflow-auto">
        {activeTab === 'spaces' && (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {techSpaces.map((space) => (
              <div
                key={space.id}
                onClick={() => setSelectedSpace(space)}
                className={`p-4 rounded-lg border cursor-pointer transition-colors ${
                  selectedSpace?.id === space.id
                    ? 'border-blue-500 bg-blue-50'
                    : 'border-gray-200 hover:border-gray-300'
                }`}
              >
                <div className="flex items-center space-x-3">
                  <span className="text-2xl">{space.icon}</span>
                  <div>
                    <h3 className="font-semibold text-gray-900">{space.name}</h3>
                    {space.description && (
                      <p className="text-sm text-gray-600">{space.description}</p>
                    )}
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}

        {activeTab === 'snippets' && selectedSpace && (
          <div className="space-y-4">
            <h2 className="text-xl font-semibold text-gray-900">
              {selectedSpace.icon} {selectedSpace.name}
            </h2>
            {snippets.map((snippet) => (
              <div key={snippet.id} className="bg-white rounded-lg border border-gray-200 p-4">
                <div className="flex items-start justify-between mb-3">
                  <div>
                    <h3 className="font-semibold text-gray-900">{snippet.title}</h3>
                    {snippet.description && (
                      <p className="text-sm text-gray-600 mt-1">{snippet.description}</p>
                    )}
                    <div className="flex items-center space-x-2 mt-2">
                      <span className="px-2 py-1 bg-gray-100 text-gray-700 text-xs rounded">
                        {snippet.language}
                      </span>
                      {snippet.tags && (
                        <span className="text-xs text-gray-500">{snippet.tags}</span>
                      )}
                    </div>
                  </div>
                  <button
                    onClick={() => copyToClipboard(snippet.code)}
                    className="px-3 py-1 bg-gray-100 text-gray-700 text-sm rounded hover:bg-gray-200 transition-colors"
                  >
                    Copy
                  </button>
                </div>
                <pre className="bg-gray-50 p-3 rounded text-sm overflow-x-auto">
                  <code>{snippet.code}</code>
                </pre>
              </div>
            ))}
          </div>
        )}

        {activeTab === 'search' && (
          <div className="space-y-4">
            <div className="flex space-x-2">
              <input
                type="text"
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                placeholder="Search code snippets..."
                className="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                onKeyPress={(e) => e.key === 'Enter' && handleSearch()}
              />
              <button
                onClick={handleSearch}
                className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
              >
                Search
              </button>
            </div>
            
            {searchResults.map((snippet) => (
              <div key={snippet.id} className="bg-white rounded-lg border border-gray-200 p-4">
                <div className="flex items-start justify-between mb-3">
                  <div>
                    <h3 className="font-semibold text-gray-900">{snippet.title}</h3>
                    {snippet.description && (
                      <p className="text-sm text-gray-600 mt-1">{snippet.description}</p>
                    )}
                    <div className="flex items-center space-x-2 mt-2">
                      <span className="px-2 py-1 bg-gray-100 text-gray-700 text-xs rounded">
                        {snippet.language}
                      </span>
                      {snippet.tags && (
                        <span className="text-xs text-gray-500">{snippet.tags}</span>
                      )}
                    </div>
                  </div>
                  <button
                    onClick={() => copyToClipboard(snippet.code)}
                    className="px-3 py-1 bg-gray-100 text-gray-700 text-sm rounded hover:bg-gray-200 transition-colors"
                  >
                    Copy
                  </button>
                </div>
                <pre className="bg-gray-50 p-3 rounded text-sm overflow-x-auto">
                  <code>{snippet.code}</code>
                </pre>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Create Space Modal */}
      {showSpaceForm && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 w-full max-w-md">
            <h2 className="text-xl font-semibold mb-4">Create Tech Space</h2>
            <form onSubmit={handleCreateSpace} className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Name</label>
                <input
                  type="text"
                  value={spaceForm.name}
                  onChange={(e) => setSpaceForm({ ...spaceForm, name: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  required
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Description</label>
                <textarea
                  value={spaceForm.description}
                  onChange={(e) => setSpaceForm({ ...spaceForm, description: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  rows={3}
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Icon</label>
                <input
                  type="text"
                  value={spaceForm.icon}
                  onChange={(e) => setSpaceForm({ ...spaceForm, icon: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  placeholder="💻"
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
                  onClick={() => setShowSpaceForm(false)}
                  className="flex-1 px-4 py-2 bg-gray-300 text-gray-700 rounded-lg hover:bg-gray-400 transition-colors"
                >
                  Cancel
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      {/* Create Snippet Modal */}
      {showSnippetForm && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 w-full max-w-2xl">
            <h2 className="text-xl font-semibold mb-4">Create Code Snippet</h2>
            <form onSubmit={handleCreateSnippet} className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Title</label>
                <input
                  type="text"
                  value={snippetForm.title}
                  onChange={(e) => setSnippetForm({ ...snippetForm, title: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  required
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Description</label>
                <textarea
                  value={snippetForm.description}
                  onChange={(e) => setSnippetForm({ ...snippetForm, description: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  rows={2}
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Language</label>
                <select
                  value={snippetForm.language}
                  onChange={(e) => setSnippetForm({ ...snippetForm, language: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                >
                  <option value="javascript">JavaScript</option>
                  <option value="typescript">TypeScript</option>
                  <option value="python">Python</option>
                  <option value="rust">Rust</option>
                  <option value="java">Java</option>
                  <option value="cpp">C++</option>
                  <option value="html">HTML</option>
                  <option value="css">CSS</option>
                  <option value="sql">SQL</option>
                  <option value="bash">Bash</option>
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Code</label>
                <textarea
                  value={snippetForm.code}
                  onChange={(e) => setSnippetForm({ ...snippetForm, code: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent font-mono"
                  rows={10}
                  required
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Tags</label>
                <input
                  type="text"
                  value={snippetForm.tags}
                  onChange={(e) => setSnippetForm({ ...snippetForm, tags: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  placeholder="react, hooks, api"
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
                  onClick={() => setShowSnippetForm(false)}
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

export default TechNotesPage;
