'use client';

import { useState, useEffect } from 'react';

interface UrlEntry {
  originalUrl: string;
  shortUrl: string;
  shortCode: string;
  createdAt: string;
}

interface CopyState {
  [key: string]: boolean;
}

interface HealthStatus {
  status: 'online' | 'offline' | 'loading';
  lastChecked: Date;
}

export default function Home() {
  const [url, setUrl] = useState('');
  const [customAlias, setCustomAlias] = useState('');
  const [shortenedUrl, setShortenedUrl] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState('');
  const [urlHistory, setUrlHistory] = useState<UrlEntry[]>([]);
  const [copiedStates, setCopiedStates] = useState<CopyState>({});
  const [healthStatus, setHealthStatus] = useState<HealthStatus>({
    status: 'loading',
    lastChecked: new Date(),
  });
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  const checkHealth = async () => {
    try {
      const response = await fetch('http://localhost:8080/api/v1/health');
      const data = await response.json();
      setHealthStatus({
        status: data.status === 'ok' ? 'online' : 'offline',
        lastChecked: new Date(),
      });
    } catch {
      setHealthStatus({
        status: 'offline',
        lastChecked: new Date(),
      });
    }
  };

  useEffect(() => {
    // Initial health check
    checkHealth();

    // Set up periodic health checks every 30 seconds
    const healthCheckInterval = setInterval(checkHealth, 30000);

    return () => clearInterval(healthCheckInterval);
  }, []);

  useEffect(() => {
    // Load URL history from localStorage on component mount
    const savedHistory = localStorage.getItem('urlHistory');
    if (savedHistory) {
      setUrlHistory(JSON.parse(savedHistory));
    }
  }, []);

  const handleCopy = async (text: string) => {
    try {
      await navigator.clipboard.writeText(text);
      setCopiedStates((prev) => ({ ...prev, [text]: true }));
      setTimeout(() => {
        setCopiedStates((prev) => ({ ...prev, [text]: false }));
      }, 2000);
    } catch (err) {
      console.error('Failed to copy:', err);
    }
  };

  const addToHistory = (entry: UrlEntry) => {
    const newHistory = [entry, ...urlHistory].slice(0, 10); // Keep only last 10 entries
    setUrlHistory(newHistory);
    localStorage.setItem('urlHistory', JSON.stringify(newHistory));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsLoading(true);
    setError('');

    try {
      const response = await fetch('http://localhost:8080/api/v1/shorten', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          url,
          custom_alias: customAlias || undefined,
        }),
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.error || 'Failed to shorten URL');
      }

      const data = await response.json();
      const newEntry: UrlEntry = {
        originalUrl: data.original_url,
        shortUrl: data.short_url,
        shortCode: data.short_code,
        createdAt: new Date().toISOString(),
      };

      setShortenedUrl(data.short_url);
      addToHistory(newEntry);
      setUrl(''); // Clear input after successful submission
      setCustomAlias(''); // Clear custom alias after successful submission
    } catch (err: unknown) {
      setError(
        err instanceof Error
          ? err.message
          : 'Failed to shorten URL. Please try again.'
      );
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <main className="min-h-screen flex flex-col items-center justify-center py-12 px-4">
      <div className="w-full max-w-xl">
        <div className="text-center mb-8">
          <h1 className="text-4xl font-bold mb-1">
            Blazing Fast URL Shortener ⚡
          </h1>
          <div className="flex items-center justify-center gap-2 text-gray-600">
            <p>Transform your long URLs into short links</p>
            <div
              className={`flex items-center gap-1.5 px-2 py-0.5 rounded-full text-xs font-medium border
                ${
                  healthStatus.status === 'online'
                    ? 'border-green-200 bg-green-50 text-green-700'
                    : healthStatus.status === 'offline'
                    ? 'border-red-200 bg-red-50 text-red-700'
                    : 'border-gray-200 bg-gray-50 text-gray-700'
                }`}
              {...(mounted && {
                title: `Last checked: ${healthStatus.lastChecked.toLocaleTimeString()}`,
              })}
            >
              <span
                className={`w-1.5 h-1.5 rounded-full ${
                  healthStatus.status === 'online'
                    ? 'bg-green-500'
                    : healthStatus.status === 'offline'
                    ? 'bg-red-500'
                    : 'bg-gray-500'
                }`}
              />
              <span className="capitalize leading-none">
                {healthStatus.status}
              </span>
            </div>
          </div>
        </div>

        <div className="bg-white shadow-sm border border-gray-200 rounded-lg p-5 mb-6">
          <form onSubmit={handleSubmit} className="space-y-3">
            <div className="space-y-3">
              <div>
                <label
                  htmlFor="url"
                  className="block text-sm font-medium text-gray-700 mb-1"
                >
                  URL to Shorten
                </label>
                <input
                  id="url"
                  type="url"
                  value={url}
                  onChange={(e) => setUrl(e.target.value)}
                  placeholder="Enter your URL here"
                  required
                  className="w-full px-3 py-2 rounded-md border border-gray-200 
                         bg-white focus:ring-2 focus:ring-blue-500 
                         focus:border-transparent outline-none transition-all"
                />
              </div>

              <div>
                <label
                  htmlFor="alias"
                  className="block text-sm font-medium text-gray-700 mb-1"
                >
                  Custom Alias{' '}
                  <span className="text-gray-500 font-normal">(optional)</span>
                </label>
                <input
                  id="alias"
                  type="text"
                  value={customAlias}
                  onChange={(e) => setCustomAlias(e.target.value)}
                  placeholder="e.g., my-custom-link"
                  pattern="[a-zA-Z0-9\-_]+"
                  title="Only letters, numbers, hyphens, and underscores are allowed"
                  className="w-full px-3 py-2 rounded-md border border-gray-200 
                         bg-white focus:ring-2 focus:ring-blue-500 
                         focus:border-transparent outline-none transition-all"
                />
              </div>
            </div>

            <button
              type="submit"
              disabled={isLoading}
              className="w-full bg-blue-500 hover:bg-blue-600 text-white font-medium 
                     py-2 px-4 rounded-md transition-colors disabled:opacity-50 
                     disabled:cursor-not-allowed focus:outline-none focus:ring-2 
                     focus:ring-offset-2 focus:ring-blue-500"
            >
              {isLoading ? 'Shortening...' : 'Shorten URL'}
            </button>
          </form>
        </div>

        {error && (
          <div className="mb-4 p-3 text-sm text-red-600 bg-red-50 border border-red-100 rounded-md">
            {error}
          </div>
        )}

        {shortenedUrl && (
          <div className="mb-6 p-4 bg-blue-50 border border-blue-100 rounded-md">
            <p className="text-sm font-medium text-blue-700 mb-2">
              Your shortened URL is ready!
            </p>
            <div className="flex items-center justify-between gap-3">
              <a
                href={shortenedUrl}
                target="_blank"
                rel="noopener noreferrer"
                className="text-blue-600 truncate hover:underline"
              >
                {shortenedUrl}
              </a>
              <button
                onClick={() => handleCopy(shortenedUrl)}
                className={`shrink-0 inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md
                       transition-all duration-200 ${
                         copiedStates[shortenedUrl]
                           ? 'bg-green-100 text-green-700'
                           : 'hover:bg-gray-100 text-gray-600'
                       }`}
                title="Copy to clipboard"
              >
                {copiedStates[shortenedUrl] ? (
                  <>
                    <svg
                      className="w-4 h-4"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M5 13l4 4L19 7"
                      />
                    </svg>
                    <span className="text-sm">Copied!</span>
                  </>
                ) : (
                  <>
                    <svg
                      className="w-4 h-4"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"
                      />
                    </svg>
                    <span className="text-sm">Copy</span>
                  </>
                )}
              </button>
            </div>
          </div>
        )}

        {urlHistory.length > 0 && (
          <div className="bg-white shadow-sm border border-gray-200 rounded-lg p-5">
            <h2 className="text-lg font-medium mb-3">Recent URLs</h2>
            <div className="max-h-[320px] overflow-y-auto hide-scrollbar">
              <div className="space-y-2 pr-1">
                {urlHistory.map((entry, index) => (
                  <div
                    key={index}
                    className="p-2.5 bg-gray-50 rounded-md hover:bg-gray-100 transition-colors group border border-gray-200"
                  >
                    <div className="flex items-start justify-between gap-3">
                      <div className="min-w-0 flex-1">
                        <div className="flex items-center gap-2">
                          <span className="text-xs font-medium text-gray-500 shrink-0">
                            Short:
                          </span>
                          <a
                            href={entry.shortUrl}
                            target="_blank"
                            rel="noopener noreferrer"
                            className="text-blue-600 font-medium truncate hover:underline"
                          >
                            {entry.shortUrl}
                          </a>
                        </div>
                        <div className="flex items-center gap-2 mt-0.5">
                          <span className="text-xs font-medium text-gray-500 shrink-0">
                            URL:
                          </span>
                          <p className="text-sm text-gray-600 truncate">
                            {entry.originalUrl}
                          </p>
                        </div>
                      </div>
                      <button
                        onClick={() => handleCopy(entry.shortUrl)}
                        className={`shrink-0 p-1.5 rounded-md transition-all duration-200 
                                ${
                                  copiedStates[entry.shortUrl]
                                    ? 'text-green-600 bg-green-50'
                                    : 'text-gray-400 hover:text-gray-600 opacity-0 group-hover:opacity-100'
                                }`}
                        title={
                          copiedStates[entry.shortUrl]
                            ? 'Copied!'
                            : 'Copy shortened URL'
                        }
                      >
                        {copiedStates[entry.shortUrl] ? (
                          <svg
                            className="w-4 h-4"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                          >
                            <path
                              strokeLinecap="round"
                              strokeLinejoin="round"
                              strokeWidth={2}
                              d="M5 13l4 4L19 7"
                            />
                          </svg>
                        ) : (
                          <svg
                            className="w-4 h-4"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                          >
                            <path
                              strokeLinecap="round"
                              strokeLinejoin="round"
                              strokeWidth={2}
                              d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"
                            />
                          </svg>
                        )}
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>
        )}
      </div>
    </main>
  );
}
