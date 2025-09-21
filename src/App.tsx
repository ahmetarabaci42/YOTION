import { useState } from 'react';
import LanguagePage from './pages/LanguagePage';
import TechNotesPage from './pages/TechNotesPage';
import ProjectsPage from './pages/ProjectsPage';
import PlannerPage from './pages/PlannerPage';
import PersonalVaultPage from './pages/PersonalVaultPage';

export type Page = 'language' | 'tech' | 'projects' | 'planner' | 'vault';

function App() {
  const [currentPage, setCurrentPage] = useState<Page>('language');

  const renderPage = () => {
    switch (currentPage) {
      case 'language':
        return <LanguagePage />;
      case 'tech':
        return <TechNotesPage />;
      case 'projects':
        return <ProjectsPage />;
      case 'planner':
        return <PlannerPage />;
      case 'vault':
        return <PersonalVaultPage />;
      default:
        return <LanguagePage />;
    }
  };

  return (
    <div className="flex h-screen bg-notion-bg">
      <div className="w-64 bg-white border-r border-notion-border h-full flex flex-col">
        <div className="p-6">
          <h1 className="text-2xl font-bold text-notion-text">YOTION</h1>
          <p className="text-sm text-notion-muted mt-1">Local Knowledge Base</p>
        </div>
        
        <nav className="flex-1 px-4">
          <ul className="space-y-2">
            <li>
              <button
                onClick={() => setCurrentPage('language')}
                className={`sidebar-item w-full text-left ${
                  currentPage === 'language' ? 'active' : ''
                }`}
              >
                <span className="mr-3 text-lg">🌍</span>
                <span>Languages</span>
              </button>
            </li>
            <li>
              <button
                onClick={() => setCurrentPage('tech')}
                className={`sidebar-item w-full text-left ${
                  currentPage === 'tech' ? 'active' : ''
                }`}
              >
                <span className="mr-3 text-lg">💻</span>
                <span>Tech Notes</span>
              </button>
            </li>
            <li>
              <button
                onClick={() => setCurrentPage('projects')}
                className={`sidebar-item w-full text-left ${
                  currentPage === 'projects' ? 'active' : ''
                }`}
              >
                <span className="mr-3 text-lg">📋</span>
                <span>Projects</span>
              </button>
            </li>
            <li>
              <button
                onClick={() => setCurrentPage('planner')}
                className={`sidebar-item w-full text-left ${
                  currentPage === 'planner' ? 'active' : ''
                }`}
              >
                <span className="mr-3 text-lg">📅</span>
                <span>Planner</span>
              </button>
            </li>
            <li>
              <button
                onClick={() => setCurrentPage('vault')}
                className={`sidebar-item w-full text-left ${
                  currentPage === 'vault' ? 'active' : ''
                }`}
              >
                <span className="mr-3 text-lg">🔐</span>
                <span>Personal Vault</span>
              </button>
            </li>
          </ul>
        </nav>
        
        <div className="p-4 border-t border-notion-border">
          <div className="text-xs text-notion-muted">
            Local-only • Offline-first
          </div>
        </div>
      </div>
      
      <main className="flex-1 overflow-hidden">
        <div className="p-8 h-full overflow-auto">
          <div className="max-w-6xl mx-auto">
            <h2 className="text-3xl font-bold mb-8">
              {currentPage === 'language' && 'Language Learning'}
              {currentPage === 'tech' && 'Tech Notes'}
              {currentPage === 'projects' && 'Projects'}
              {currentPage === 'planner' && 'Planner'}
              {currentPage === 'vault' && 'Personal Vault'}
            </h2>
            
            {renderPage()}
          </div>
        </div>
      </main>
    </div>
  );
}

export default App;
