'use client';

import { useState, useEffect, useCallback } from 'react';
import { 
  fetchEntries, 
  createEntryWithTags, 
  fetchTags, 
  fetchEntriesByTag,
  EntryWithTags,
  Tag
} from '@/lib/api';
import { format } from 'date-fns';
import { ja } from 'date-fns/locale';

export default function Home() {
  const [entries, setEntries] = useState<EntryWithTags[]>([]);
  const [content, setContent] = useState('');
  const [currentPage, setCurrentPage] = useState(1);
  const [totalPages, setTotalPages] = useState(1);
  const [isLoading, setIsLoading] = useState(false);
  
  // タグ関連の状態
  const [allTags, setAllTags] = useState<Tag[]>([]);
  const [selectedTags, setSelectedTags] = useState<string[]>([]);
  const [tagInput, setTagInput] = useState('');
  const [filterTagId, setFilterTagId] = useState<number | null>(null);

  // タグの読み込み
  const loadTags = useCallback(async () => {
    try {
      const tags = await fetchTags();
      setAllTags(tags);
    } catch (error) {
      console.error('Failed to fetch tags:', error);
    }
  }, []);

  // エントリの読み込み（タグフィルター対応）
  const loadEntries = useCallback(async (page: number) => {
    setIsLoading(true);
    try {
      let data;
      if (filterTagId !== null) {
        // 特定のタグでフィルタリング
        data = await fetchEntriesByTag(filterTagId, page);
      } else {
        // すべてのエントリを取得
        data = await fetchEntries(page);
      }
      setEntries(data.entries);
      setTotalPages(data.total_pages);
      setCurrentPage(data.current_page);
    } catch (error) {
      console.error('Failed to fetch entries:', error);
    } finally {
      setIsLoading(false);
    }
  }, [filterTagId]);

  useEffect(() => {
    loadTags();
  }, [loadTags]);

  useEffect(() => {
    loadEntries(currentPage);
  }, [currentPage, loadEntries]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!content.trim()) return;

    setIsLoading(true);
    try {
      await createEntryWithTags(content, selectedTags);
      setContent('');
      setSelectedTags([]);
      setTagInput('');
      loadEntries(1); // 投稿後は1ページ目に戻る
      loadTags(); // タグリストを更新
    } catch (error) {
      console.error('Failed to create entry:', error);
    } finally {
      setIsLoading(false);
    }
  };

  // タグ入力の処理
  const handleTagInputKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter' || e.key === ',') {
      e.preventDefault();
      addTag();
    }
  };

  const addTag = () => {
    const trimmedTag = tagInput.trim();
    if (trimmedTag && !selectedTags.includes(trimmedTag)) {
      setSelectedTags([...selectedTags, trimmedTag]);
    }
    setTagInput('');
  };

  const removeTag = (tagToRemove: string) => {
    setSelectedTags(selectedTags.filter(tag => tag !== tagToRemove));
  };

  // タグフィルターの処理
  const handleTagFilter = (tagId: number | null) => {
    setFilterTagId(tagId);
    setCurrentPage(1); // フィルター変更時は1ページ目に戻る
  };

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    return format(date, 'yyyy-MM-dd HH:mm:ss', { locale: ja });
  };

  return (
    <main className="container mx-auto px-4 py-8">
      <header className="bg-green-100 dark:bg-gray-800 p-6 mb-8 rounded-lg">
        <h1 className="text-3xl font-bold">ほぼ日だいあり</h1>
      </header>

      <div className="grid grid-cols-1 md:grid-cols-4 gap-8">
        {/* サイドバー：タグフィルター */}
        <div className="md:col-span-1">
          <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow mb-6">
            <h2 className="text-xl font-semibold mb-4">タグで絞り込み</h2>
            <div className="space-y-2">
              <button
                onClick={() => handleTagFilter(null)}
                className={`block w-full text-left px-3 py-2 rounded ${
                  filterTagId === null ? 'bg-blue-100 dark:bg-blue-900' : 'hover:bg-gray-100 dark:hover:bg-gray-700'
                }`}
              >
                すべて表示
              </button>
              {allTags.map(tag => (
                <button
                  key={tag.id}
                  onClick={() => handleTagFilter(tag.id)}
                  className={`block w-full text-left px-3 py-2 rounded ${
                    filterTagId === tag.id ? 'bg-blue-100 dark:bg-blue-900' : 'hover:bg-gray-100 dark:hover:bg-gray-700'
                  }`}
                >
                  #{tag.name}
                </button>
              ))}
            </div>
          </div>
        </div>

        {/* メインコンテンツ */}
        <div className="md:col-span-3">
          <form onSubmit={handleSubmit} className="mb-8 bg-white dark:bg-gray-800 p-6 rounded-lg shadow">
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
            
            {/* タグ入力エリア */}
            <div className="mb-4">
              <div className="flex flex-wrap gap-2 mb-2">
                {selectedTags.map(tag => (
                  <span 
                    key={tag} 
                    className="bg-blue-100 text-blue-800 px-2 py-1 rounded-full text-sm flex items-center"
                  >
                    #{tag}
                    <button 
                      type="button"
                      onClick={() => removeTag(tag)}
                      className="ml-1 text-blue-500 hover:text-blue-700"
                    >
                      ×
                    </button>
                  </span>
                ))}
              </div>
              <div className="flex">
                <input
                  type="text"
                  className="flex-grow p-2 border rounded-l-md"
                  placeholder="タグを追加（カンマまたはEnterで区切り）"
                  value={tagInput}
                  onChange={(e) => setTagInput(e.target.value)}
                  onKeyDown={handleTagInputKeyDown}
                  disabled={isLoading}
                />
                <button
                  type="button"
                  onClick={addTag}
                  className="px-4 py-2 bg-gray-200 text-gray-800 rounded-r-md hover:bg-gray-300 disabled:bg-gray-100"
                  disabled={isLoading || !tagInput.trim()}
                >
                  追加
                </button>
              </div>
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
                <div key={entry.id} className="mb-6 border rounded-lg shadow overflow-hidden bg-white dark:bg-gray-800">
                  <div className="bg-gray-100 dark:bg-gray-700 p-3 border-b flex justify-between items-center">
                    <span>{formatDate(entry.datetime)}</span>
                  </div>
                  <div className="p-4 whitespace-pre-wrap">
                    {entry.content.split('\n').map((line, i) => (
                      <p key={i}>{line}</p>
                    ))}
                  </div>
                  {entry.tags && entry.tags.length > 0 && (
                    <div className="px-4 pb-4 pt-2 border-t flex flex-wrap gap-2">
                      {entry.tags.map(tag => (
                        <span 
                          key={tag.id} 
                          className="bg-blue-100 text-blue-800 px-2 py-1 rounded-full text-sm cursor-pointer"
                          onClick={() => handleTagFilter(tag.id)}
                        >
                          #{tag.name}
                        </span>
                      ))}
                    </div>
                  )}
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
            <div className="text-center p-8 bg-gray-50 dark:bg-gray-800 rounded-md">
              {isLoading ? '読み込み中...' : '日記がまだありません。最初の日記を投稿してみましょう！'}
            </div>
          )}
        </div>
      </div>
    </main>
  );
}
