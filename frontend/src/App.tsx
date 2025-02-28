import React, { useEffect, useState } from 'react';
import { getTasks, createTask, updateTask, deleteTask, Task } from './api';
import TaskList from './components/TaskList';
import TaskForm from './components/TaskForm';
import { logger } from './utils/logger';

const App: React.FC = () => {
  const [tasks, setTasks] = useState<Task[]>([]);

  const fetchTasks = async () => {
    try {
      logger.info('Initializing task fetch...');
      const data = await getTasks();
      setTasks(data);
    } catch (error) {
      logger.error('Error in component while fetching tasks:', error);
    }
  };

  useEffect(() => {
    logger.info('App component mounted');
    fetchTasks();
    return () => {
      logger.info('App component unmounted');
    };
  }, []);

  const handleAddTask = async (title: string) => {
    try {
      logger.info('Adding new task:', { title });
      const task = await createTask(title);
      setTasks((prev) => [...prev, task]);
    } catch (error) {
      logger.error('Error in component while adding task:', error);
    }
  };

  const handleUpdateTask = async (id: number, title: string) => {
    try {
      logger.info('Updating task:', { id, title });
      const updatedTask = await updateTask(id, title);
      setTasks((prev) => prev.map((task) => (task.id === id ? updatedTask : task)));
    } catch (error) {
      logger.error('Error in component while updating task:', error);
    }
  };

  const handleDeleteTask = async (id: number) => {
    try {
      logger.info('Deleting task:', { id });
      await deleteTask(id);
      setTasks((prev) => prev.filter((task) => task.id !== id));
    } catch (error) {
      logger.error('Error in component while deleting task:', error);
    }
  };

  return (
    <div className="container">
      <h1>Task List</h1>
      <TaskForm onAddTask={handleAddTask} />
      <TaskList tasks={tasks} onUpdateTask={handleUpdateTask} onDeleteTask={handleDeleteTask} />
    </div>
  );
};

export default App;
