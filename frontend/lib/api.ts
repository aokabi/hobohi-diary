import axios from 'axios';

const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:9001/api';

export interface Entry {
  id: number;
  content: string;
  datetime: string;
}

export interface EntriesResponse {
  entries: Entry[];
  total_pages: number;
  current_page: number;
}

export const fetchEntries = async (page: number = 1): Promise<EntriesResponse> => {
  const response = await axios.get(`${API_URL}/entries?page=${page}`);
  return response.data;
};

export const createEntry = async (content: string): Promise<void> => {
  await axios.post(`${API_URL}/entries`, { content });
};

export const fetchEntryCount = async (): Promise<number> => {
  const response = await axios.get(`${API_URL}/entries/count`);
  return response.data;
};
