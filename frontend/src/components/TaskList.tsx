import React from 'react'
import { Task } from '../api'
import TaskItem from './TaskItem'

interface TaskListProps {
  tasks: Task[]
  onUpdateTask: (id: number, title: string) => void
  onDeleteTask: (id: number) => void
}

const TaskList: React.FC<TaskListProps> = ({ tasks, onUpdateTask, onDeleteTask }) => {
  return (
    <ul>
      {tasks.map(task => (
        <TaskItem 
          key={task.id} 
          task={task} 
          onUpdateTask={onUpdateTask} 
          onDeleteTask={onDeleteTask} 
        />
      ))}
    </ul>
  )
}

export default TaskList
