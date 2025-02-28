import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, beforeAll, afterAll, afterEach } from 'vitest';
import App from '../src/App';
import { setupServer } from 'msw/node';
import { http, HttpResponse } from 'msw';

interface TaskData {
  title: string;
}

// Mock data
const tasks = [
  { id: 1, title: 'Test Task 1' },
  { id: 2, title: 'Test Task 2' },
];

// Set up an MSW server to mock API responses
const server = setupServer(
  http.get('http://127.0.0.1:8000/tasks', () => {
    return HttpResponse.json(tasks);
  }),
  http.post('http://127.0.0.1:8000/tasks', async ({ request }) => {
    const data = (await request.json()) as TaskData;
    const newTask = { id: 3, title: data.title };
    return HttpResponse.json(newTask);
  }),
  http.put('http://127.0.0.1:8000/tasks/:id', async ({ params, request }) => {
    const { id } = params;
    const data = (await request.json()) as TaskData;
    return HttpResponse.json({ id: Number(id), title: data.title });
  }),
  http.delete('http://127.0.0.1:8000/tasks/:id', () => {
    return new HttpResponse(null, { status: 200 });
  })
);

beforeAll(() => server.listen());
afterEach(() => server.resetHandlers());
afterAll(() => server.close());

describe('App', () => {
  it('renders Task List and adds a new task', async () => {
    render(<App />);

    // Wait for tasks to load
    await waitFor(() => {
      expect(screen.getByText('Test Task 1')).toBeTruthy();
    });

    // Add a new task
    const input = screen.getByPlaceholderText('New Task');
    fireEvent.change(input, { target: { value: 'New Test Task' } });
    fireEvent.click(screen.getByText('Add Task'));

    // Wait for the new task to appear
    await waitFor(() => {
      expect(screen.getByText('New Test Task')).toBeTruthy();
    });
  });
});
