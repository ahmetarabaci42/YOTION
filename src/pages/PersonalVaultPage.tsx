import React, { useState, useEffect } from 'react';
import { 
  PersonalAccount, 
  PersonalInfo, 
  createPersonalAccount, 
  getPersonalAccounts,
  getPersonalAccountsByCategory,
  createPersonalInfo,
  getPersonalInfo,
  getPersonalInfoByCategory
} from '../api/tauri';

const PersonalVaultPage: React.FC = () => {
  const [activeTab, setActiveTab] = useState<'accounts' | 'info'>('accounts');
  const [accounts, setAccounts] = useState<PersonalAccount[]>([]);
  const [personalInfo, setPersonalInfo] = useState<PersonalInfo[]>([]);
  const [selectedCategory, setSelectedCategory] = useState<string>('all');
  const [showPassword, setShowPassword] = useState<{ [key: number]: boolean }>({});
  
  // Form states
  const [showAccountForm, setShowAccountForm] = useState(false);
  const [showInfoForm, setShowInfoForm] = useState(false);
  const [accountForm, setAccountForm] = useState({
    title: '',
    email: '',
    password: '',
    website: '',
    notes: '',
    category: 'email'
  });
  const [infoForm, setInfoForm] = useState({
    title: '',
    content: '',
    category: 'identity',
    is_sensitive: false
  });

  const accountCategories = [
    { value: 'email', label: 'Email', icon: '📧' },
    { value: 'social', label: 'Social Media', icon: '📱' },
    { value: 'banking', label: 'Banking', icon: '🏦' },
    { value: 'work', label: 'Work', icon: '💼' },
    { value: 'shopping', label: 'Shopping', icon: '🛒' },
    { value: 'entertainment', label: 'Entertainment', icon: '🎬' },
    { value: 'other', label: 'Other', icon: '📝' }
  ];

  const infoCategories = [
    { value: 'identity', label: 'Identity', icon: '🆔' },
    { value: 'documents', label: 'Documents', icon: '📄' },
    { value: 'contacts', label: 'Contacts', icon: '👥' },
    { value: 'medical', label: 'Medical', icon: '🏥' },
    { value: 'insurance', label: 'Insurance', icon: '🛡️' },
    { value: 'other', label: 'Other', icon: '📋' }
  ];

  useEffect(() => {
    loadAccounts();
    loadPersonalInfo();
  }, []);

  useEffect(() => {
    if (selectedCategory === 'all') {
      loadAccounts();
    } else {
      loadAccountsByCategory(selectedCategory);
    }
  }, [selectedCategory]);

  const loadAccounts = async () => {
    try {
      const accountList = await getPersonalAccounts();
      setAccounts(accountList);
    } catch (error) {
      console.error('Failed to load accounts:', error);
    }
  };

  const loadAccountsByCategory = async (category: string) => {
    try {
      const accountList = await getPersonalAccountsByCategory(category);
      setAccounts(accountList);
    } catch (error) {
      console.error('Failed to load accounts by category:', error);
    }
  };

  const loadPersonalInfo = async () => {
    try {
      const infoList = await getPersonalInfo();
      setPersonalInfo(infoList);
    } catch (error) {
      console.error('Failed to load personal info:', error);
    }
  };

  const handleCreateAccount = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await createPersonalAccount(accountForm);
      setAccountForm({
        title: '',
        email: '',
        password: '',
        website: '',
        notes: '',
        category: 'email'
      });
      setShowAccountForm(false);
      loadAccounts();
    } catch (error) {
      console.error('Failed to create account:', error);
    }
  };

  const handleCreateInfo = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await createPersonalInfo(infoForm);
      setInfoForm({
        title: '',
        content: '',
        category: 'identity',
        is_sensitive: false
      });
      setShowInfoForm(false);
      loadPersonalInfo();
    } catch (error) {
      console.error('Failed to create personal info:', error);
    }
  };

  const togglePasswordVisibility = (accountId: number) => {
    setShowPassword(prev => ({
      ...prev,
      [accountId]: !prev[accountId]
    }));
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  const getCategoryIcon = (category: string, type: 'account' | 'info') => {
    const categories = type === 'account' ? accountCategories : infoCategories;
    const found = categories.find(cat => cat.value === category);
    return found ? found.icon : '📝';
  };

  const getCategoryLabel = (category: string, type: 'account' | 'info') => {
    const categories = type === 'account' ? accountCategories : infoCategories;
    const found = categories.find(cat => cat.value === category);
    return found ? found.label : category;
  };

  return (
    <div className="h-full flex flex-col">
      {/* Header */}
      <div className="flex items-center justify-between p-6 border-b border-gray-200">
        <div>
          <h1 className="text-2xl font-semibold text-gray-900">🔐 Personal Vault</h1>
          <p className="text-sm text-gray-600 mt-1">Secure storage for your personal information</p>
        </div>
        <div className="flex space-x-2">
          <button
            onClick={() => setShowAccountForm(true)}
            className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
          >
            New Account
          </button>
          <button
            onClick={() => setShowInfoForm(true)}
            className="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors"
          >
            New Info
          </button>
        </div>
      </div>

      {/* Tabs */}
      <div className="flex border-b border-gray-200">
        <button
          onClick={() => setActiveTab('accounts')}
          className={`px-6 py-3 font-medium ${
            activeTab === 'accounts'
              ? 'text-blue-600 border-b-2 border-blue-600'
              : 'text-gray-500 hover:text-gray-700'
          }`}
        >
          📧 Accounts & Passwords
        </button>
        <button
          onClick={() => setActiveTab('info')}
          className={`px-6 py-3 font-medium ${
            activeTab === 'info'
              ? 'text-blue-600 border-b-2 border-blue-600'
              : 'text-gray-500 hover:text-gray-700'
          }`}
        >
          📋 Personal Information
        </button>
      </div>

      {/* Content */}
      <div className="flex-1 p-6 overflow-auto">
        {activeTab === 'accounts' && (
          <div className="space-y-6">
            {/* Category Filter */}
            <div className="flex flex-wrap gap-2">
              <button
                onClick={() => setSelectedCategory('all')}
                className={`px-3 py-1 rounded-full text-sm ${
                  selectedCategory === 'all'
                    ? 'bg-blue-100 text-blue-800'
                    : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                }`}
              >
                All
              </button>
              {accountCategories.map((category) => (
                <button
                  key={category.value}
                  onClick={() => setSelectedCategory(category.value)}
                  className={`px-3 py-1 rounded-full text-sm ${
                    selectedCategory === category.value
                      ? 'bg-blue-100 text-blue-800'
                      : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                  }`}
                >
                  {category.icon} {category.label}
                </button>
              ))}
            </div>

            {/* Accounts Grid */}
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              {accounts.map((account) => (
                <div key={account.id} className="bg-white rounded-lg border border-gray-200 p-4">
                  <div className="flex items-start justify-between mb-3">
                    <div className="flex items-center space-x-2">
                      <span className="text-lg">{getCategoryIcon(account.category, 'account')}</span>
                      <div>
                        <h3 className="font-semibold text-gray-900">{account.title}</h3>
                        <p className="text-xs text-gray-500">{getCategoryLabel(account.category, 'account')}</p>
                      </div>
                    </div>
                  </div>
                  
                  <div className="space-y-2">
                    <div>
                      <label className="text-xs text-gray-500">Email</label>
                      <div className="flex items-center space-x-2">
                        <p className="text-sm font-mono bg-gray-50 px-2 py-1 rounded flex-1">
                          {account.email}
                        </p>
                        <button
                          onClick={() => copyToClipboard(account.email)}
                          className="text-xs text-blue-600 hover:text-blue-800"
                        >
                          Copy
                        </button>
                      </div>
                    </div>
                    
                    <div>
                      <label className="text-xs text-gray-500">Password</label>
                      <div className="flex items-center space-x-2">
                        <p className="text-sm font-mono bg-gray-50 px-2 py-1 rounded flex-1">
                          {showPassword[account.id] ? account.password : '••••••••'}
                        </p>
                        <button
                          onClick={() => togglePasswordVisibility(account.id)}
                          className="text-xs text-blue-600 hover:text-blue-800"
                        >
                          {showPassword[account.id] ? 'Hide' : 'Show'}
                        </button>
                        <button
                          onClick={() => copyToClipboard(account.password)}
                          className="text-xs text-blue-600 hover:text-blue-800"
                        >
                          Copy
                        </button>
                      </div>
                    </div>
                    
                    {account.website && (
                      <div>
                        <label className="text-xs text-gray-500">Website</label>
                        <p className="text-sm text-blue-600 hover:underline cursor-pointer">
                          {account.website}
                        </p>
                      </div>
                    )}
                    
                    {account.notes && (
                      <div>
                        <label className="text-xs text-gray-500">Notes</label>
                        <p className="text-sm text-gray-700">{account.notes}</p>
                      </div>
                    )}
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {activeTab === 'info' && (
          <div className="space-y-4">
            {personalInfo.map((info) => (
              <div key={info.id} className="bg-white rounded-lg border border-gray-200 p-4">
                <div className="flex items-start justify-between mb-3">
                  <div className="flex items-center space-x-2">
                    <span className="text-lg">{getCategoryIcon(info.category, 'info')}</span>
                    <div>
                      <h3 className="font-semibold text-gray-900">{info.title}</h3>
                      <div className="flex items-center space-x-2">
                        <span className="text-xs text-gray-500">{getCategoryLabel(info.category, 'info')}</span>
                        {info.is_sensitive && (
                          <span className="text-xs bg-red-100 text-red-800 px-2 py-1 rounded">
                            Sensitive
                          </span>
                        )}
                      </div>
                    </div>
                  </div>
                  <button
                    onClick={() => copyToClipboard(info.content)}
                    className="text-xs text-blue-600 hover:text-blue-800"
                  >
                    Copy
                  </button>
                </div>
                
                <div className="bg-gray-50 p-3 rounded">
                  <pre className="text-sm text-gray-700 whitespace-pre-wrap font-sans">
                    {info.content}
                  </pre>
                </div>
                
                <div className="text-xs text-gray-500 mt-2">
                  Created: {new Date(info.created_at).toLocaleDateString()}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Create Account Modal */}
      {showAccountForm && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 w-full max-w-md">
            <h2 className="text-xl font-semibold mb-4">Add New Account</h2>
            <form onSubmit={handleCreateAccount} className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Title</label>
                <input
                  type="text"
                  value={accountForm.title}
                  onChange={(e) => setAccountForm({ ...accountForm, title: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  placeholder="Gmail Account"
                  required
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Email</label>
                <input
                  type="email"
                  value={accountForm.email}
                  onChange={(e) => setAccountForm({ ...accountForm, email: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  required
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Password</label>
                <input
                  type="password"
                  value={accountForm.password}
                  onChange={(e) => setAccountForm({ ...accountForm, password: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  required
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Website (Optional)</label>
                <input
                  type="url"
                  value={accountForm.website}
                  onChange={(e) => setAccountForm({ ...accountForm, website: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  placeholder="https://gmail.com"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Category</label>
                <select
                  value={accountForm.category}
                  onChange={(e) => setAccountForm({ ...accountForm, category: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                >
                  {accountCategories.map((category) => (
                    <option key={category.value} value={category.value}>
                      {category.icon} {category.label}
                    </option>
                  ))}
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Notes (Optional)</label>
                <textarea
                  value={accountForm.notes}
                  onChange={(e) => setAccountForm({ ...accountForm, notes: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  rows={3}
                />
              </div>
              <div className="flex space-x-3">
                <button
                  type="submit"
                  className="flex-1 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
                >
                  Save Account
                </button>
                <button
                  type="button"
                  onClick={() => setShowAccountForm(false)}
                  className="flex-1 px-4 py-2 bg-gray-300 text-gray-700 rounded-lg hover:bg-gray-400 transition-colors"
                >
                  Cancel
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      {/* Create Info Modal */}
      {showInfoForm && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 w-full max-w-2xl">
            <h2 className="text-xl font-semibold mb-4">Add Personal Information</h2>
            <form onSubmit={handleCreateInfo} className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Title</label>
                <input
                  type="text"
                  value={infoForm.title}
                  onChange={(e) => setInfoForm({ ...infoForm, title: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  placeholder="Passport Information"
                  required
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Category</label>
                <select
                  value={infoForm.category}
                  onChange={(e) => setInfoForm({ ...infoForm, category: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                >
                  {infoCategories.map((category) => (
                    <option key={category.value} value={category.value}>
                      {category.icon} {category.label}
                    </option>
                  ))}
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Content</label>
                <textarea
                  value={infoForm.content}
                  onChange={(e) => setInfoForm({ ...infoForm, content: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  rows={8}
                  placeholder="Enter your personal information here..."
                  required
                />
              </div>
              <div className="flex items-center">
                <input
                  type="checkbox"
                  id="is_sensitive"
                  checked={infoForm.is_sensitive}
                  onChange={(e) => setInfoForm({ ...infoForm, is_sensitive: e.target.checked })}
                  className="mr-2"
                />
                <label htmlFor="is_sensitive" className="text-sm text-gray-700">
                  Mark as sensitive information
                </label>
              </div>
              <div className="flex space-x-3">
                <button
                  type="submit"
                  className="flex-1 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
                >
                  Save Information
                </button>
                <button
                  type="button"
                  onClick={() => setShowInfoForm(false)}
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

export default PersonalVaultPage;