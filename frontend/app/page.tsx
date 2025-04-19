'use client';

import { useState, useEffect } from 'react';
import { fetchEntries, createEntry, Entry } from '@/lib/api';
import { format } from 'date-fns';
import { ja } from 'date-fns/locale';

export default function Home() {
  const [entries, setEntries] = useState<Entry[]>([]);
  const [content, setContent] = useState('');
  const [currentPage, setCurrentPage] = useState(1);
  const [totalPages, setTotalPages] = useState(1);
  const [isLoading, setIsLoading] = useState(false);

  const loadEntries = async (page: number) => {
    setIsLoading(true);
    try {
      const data = await fetchEntries(page);
      setEntries(data.entries);
      setTotalPages(data.total_pages);
      setCurrentPage(data.current_page);
    } catch (error) {
      console.error('Failed to fetch entries:', error);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    loadEntries(currentPage);
  }, [currentPage]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!content.trim()) return;

    setIsLoading(true);
    try {
      await createEntry(content);
      setContent('');
      loadEntries(1); // 投稿後は1ページ目に戻る
    } catch (error) {
      console.error('Failed to create entry:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    return format(date, 'yyyy-MM-dd HH:mm:ss', { locale: ja });
  };

  return (
    <main className="container mx-auto px-4 py-8">
      <header className="bg-green-100 p-6 mb-8 rounded-lg">
        <h1 className="text-3xl font-bold">ほぼ日だいあり</h1>
      </header>

      <div className="grid grid-cols-1 md:grid-cols-4 gap-8">
        <div className="md:col-span-3">
          <form onSubmit={handleSubmit} className="mb-8">
            <div className="mb-4">
              <textarea
                className="w-full p-2 border rounded-md"
                rows={5}
                value={content}
                onChange={(e) => setContent(e.target.value)}
                placeholder="今日の出来事を書いてみよう..."
                disabled={isLoading}
              />
            </div>
            <button
              type="submit"
              className="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 disabled:bg-gray-400"
              disabled={isLoading || !content.trim()}
            >
              投稿
            </button>
          </form>

          {entries.length > 0 ? (
            <>
              {entries.map((entry) => (
                <div key={entry.id} className="mb-6 border rounded-md overflow-hidden">
                  <div className="bg-gray-100 p-3 border-b">
                    {formatDate(entry.datetime)}
                  </div>
                  <div className="p-4 whitespace-pre-wrap">
                    {entry.content.split('\n').map((line, i) => (
                      <p key={i}>{line}</p>
                    ))}
                  </div>
                </div>
              ))}

              {totalPages > 1 && (
                <div className="flex justify-center mt-8">
                  <nav className="inline-flex">
                    <button
                      onClick={() => setCurrentPage(p => Math.max(p - 1, 1))}
                      disabled={currentPage === 1 || isLoading}
                      className="px-3 py-1 border rounded-l-md disabled:bg-gray-100"
                    >
                      &laquo;
                    </button>
                    {Array.from({ length: totalPages }, (_, i) => i + 1).map(page => (
                      <button
                        key={page}
                        onClick={() => setCurrentPage(page)}
                        disabled={isLoading}
                        className={`px-3 py-1 border-t border-b ${
                          currentPage === page ? 'bg-blue-100' : ''
                        }`}
                      >
                        {page}
                      </button>
                    ))}
                    <button
                      onClick={() => setCurrentPage(p => Math.min(p + 1, totalPages))}
                      disabled={currentPage === totalPages || isLoading}
                      className="px-3 py-1 border rounded-r-md disabled:bg-gray-100"
                    >
                      &raquo;
                    </button>
                  </nav>
                </div>
              )}
            </>
          ) : (
            <div className="text-center p-8 bg-gray-50 rounded-md">
              {isLoading ? '読み込み中...' : '日記がまだありません。最初の日記を投稿してみましょう！'}
            </div>
          )}
        </div>
      </div>
    </main>
  );
}
