import React, { useState } from 'react'
import { Task } from '../api'

interface TaskItemProps {
  task: Task
  onUpdateTask: (id: number, title: string) => void
  onDeleteTask: (id: number) => void
}

const TaskItem: React.FC<TaskItemProps> = ({ task, onUpdateTask, onDeleteTask }) => {
  const [isEditing, setIsEditing] = useState(false)
  const [title, setTitle] = useState(task.title)

  const handleSave = () => {
    onUpdateTask(task.id, title)
    setIsEditing(false)
  }

  return (
    <li>
      {isEditing ? (
        <>
          <input 
            type="text" 
            value={title} 
            onChange={(e) => setTitle(e.target.value)}
          />
          <button onClick={handleSave}>Save</button>
          <button onClick={() => setIsEditing(false)}>Cancel</button>
        </>
      ) : (
        <>
          <span>{task.title}</span>
          <button onClick={() => setIsEditing(true)}>Edit</button>
          <button onClick={() => onDeleteTask(task.id)}>Delete</button>
        </>
      )}
    </li>
  )
}

export default TaskItem
