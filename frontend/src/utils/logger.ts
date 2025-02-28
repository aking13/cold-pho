import { Task } from '../api';

type LogLevel = 'log' | 'error' | 'info' | 'warn';
type LogFunction = (...args: any[]) => void;

const isDev = process.env.NODE_ENV === 'development' || localStorage.getItem('enableDevLogs') === 'true';

const createLogFunction = (level: LogLevel): LogFunction => {
  return (...args: any[]) => {
    if (isDev) {
      const timestamp = new Date().toISOString();
      console[level](`[DEV ${level.toUpperCase()}] [${timestamp}]`, ...args);
    }
  };
};

export const logger = {
  log: createLogFunction('log'),
  error: createLogFunction('error'),
  info: createLogFunction('info'),
  warn: createLogFunction('warn'),

  // API specific logging
  api: {
    request: (method: string, url: string, body?: any) => {
      logger.info(`API Request: ${method} ${url}`, body ? { body } : '');
    },
    response: (method: string, url: string, data: any) => {
      logger.info(`API Response: ${method} ${url}`, { data });
    },
    error: (method: string, url: string, error: any) => {
      logger.error(`API Error: ${method} ${url}`, { error });
    },
  },

  // Task specific logging
  tasks: {
    fetched: (tasks: Task[]) => {
      logger.info('Tasks fetched:', { count: tasks.length, tasks });
    },
    created: (task: Task) => {
      logger.info('Task created:', { task });
    },
    updated: (taskId: number, newTitle: string) => {
      logger.info('Task updated:', { taskId, newTitle });
    },
    deleted: (taskId: number) => {
      logger.info('Task deleted:', { taskId });
    },
  },

  // Toggle dev logs manually (useful for testing in production)
  enableDevLogs: () => {
    localStorage.setItem('enableDevLogs', 'true');
    console.log('Developer logs enabled');
  },
  disableDevLogs: () => {
    localStorage.removeItem('enableDevLogs');
    console.log('Developer logs disabled');
  },
};
