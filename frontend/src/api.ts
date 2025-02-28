import { logger } from './utils/logger';

export interface Task {
  id: number;
  title: string;
}

const API_URL = import.meta.env.VITE_API_URL;

export const getTasks = async (): Promise<Task[]> => {
  const url = `${API_URL}/tasks`;
  logger.api.request('GET', url);

  try {
    const response = await fetch(url);
    if (!response.ok) {
      const error = await response.text();
      logger.api.error('GET', url, { status: response.status, error });
      throw new Error('Failed to fetch tasks');
    }
    const data = await response.json();
    logger.api.response('GET', url, data);
    logger.tasks.fetched(data);
    return data;
  } catch (error) {
    logger.api.error('GET', url, error);
    throw error;
  }
};

export const createTask = async (title: string): Promise<Task> => {
  const url = `${API_URL}/tasks`;
  const body = { title };
  logger.api.request('POST', url, body);

  try {
    const response = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });
    if (!response.ok) {
      const error = await response.text();
      logger.api.error('POST', url, { status: response.status, error });
      throw new Error('Failed to create task');
    }
    const data = await response.json();
    logger.api.response('POST', url, data);
    logger.tasks.created(data);
    return data;
  } catch (error) {
    logger.api.error('POST', url, error);
    throw error;
  }
};

export const updateTask = async (id: number, title: string): Promise<Task> => {
  const url = `${API_URL}/tasks/${id}`;
  const body = { title };
  logger.api.request('PUT', url, body);

  try {
    const response = await fetch(url, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });
    if (!response.ok) {
      const error = await response.text();
      logger.api.error('PUT', url, { status: response.status, error });
      throw new Error('Failed to update task');
    }
    const data = await response.json();
    logger.api.response('PUT', url, data);
    logger.tasks.updated(id, title);
    return data;
  } catch (error) {
    logger.api.error('PUT', url, error);
    throw error;
  }
};

export const deleteTask = async (id: number): Promise<void> => {
  const url = `${API_URL}/tasks/${id}`;
  logger.api.request('DELETE', url);

  try {
    const response = await fetch(url, {
      method: 'DELETE',
    });
    if (!response.ok) {
      const error = await response.text();
      logger.api.error('DELETE', url, { status: response.status, error });
      throw new Error('Failed to delete task');
    }
    logger.api.response('DELETE', url, { success: true });
    logger.tasks.deleted(id);
  } catch (error) {
    logger.api.error('DELETE', url, error);
    throw error;
  }
};
