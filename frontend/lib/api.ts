import axios from 'axios';

const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:9001/api';

export interface Tag {
  id: number;
  name: string;
}

export interface Entry {
  id: number;
  content: string;
  datetime: string;
}

export interface EntryWithTags {
  id: number;
  content: string;
  datetime: string;
  tags: Tag[];
}

export interface EntriesResponse {
  entries: Entry[];
  total_pages: number;
  current_page: number;
}

export interface EntriesWithTagsResponse {
  entries: EntryWithTags[];
  total_pages: number;
  current_page: number;
}

export interface TagsResponse {
  tags: Tag[];
}

export const fetchEntries = async (page: number = 1): Promise<EntriesWithTagsResponse> => {
  const response = await axios.get(`${API_URL}/entries?page=${page}`);
  return response.data;
};

export const createEntry = async (content: string): Promise<void> => {
  await axios.post(`${API_URL}/entries`, { content });
};

export const createEntryWithTags = async (content: string, tags: string[]): Promise<void> => {
  await axios.post(`${API_URL}/entries/with-tags`, { content, tags });
};

export const fetchEntryCount = async (): Promise<number> => {
  const response = await axios.get(`${API_URL}/entries/count`);
  return response.data;
};

// タグ関連のAPI関数
export const fetchTags = async (): Promise<Tag[]> => {
  const response = await axios.get<TagsResponse>(`${API_URL}/tags`);
  return response.data.tags;
};

export const createTag = async (name: string): Promise<Tag> => {
  const response = await axios.post<Tag>(`${API_URL}/tags`, { name });
  return response.data;
};

export const fetchEntriesByTag = async (tagId: number, page: number = 1): Promise<EntriesWithTagsResponse> => {
  const response = await axios.get(`${API_URL}/tags/${tagId}/entries?page=${page}`);
  return response.data;
};
