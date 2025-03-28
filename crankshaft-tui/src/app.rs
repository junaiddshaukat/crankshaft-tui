//! Application state and logic for the TUI.

use crossterm::event::{KeyCode, KeyEvent};
use std::collections::HashMap;

/// Task status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "Pending"),
            TaskStatus::Running => write!(f, "Running"),
            TaskStatus::Completed => write!(f, "Completed"),
            TaskStatus::Failed => write!(f, "Failed"),
        }
    }
}

/// Represents a task in the system
#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub status: TaskStatus,
    pub progress: f64, // 0.0 to 1.0
    pub cpu_usage: f64,
    pub memory_usage: f64,
}

/// Main application state
pub struct App {
    pub tasks: HashMap<String, Task>,
    pub selected_task_id: Option<String>,
    pub task_ids: Vec<String>,
    pub should_quit: bool,
    pub tab_index: usize,
}

impl Default for App {
    fn default() -> Self {
        // Create some sample tasks for demonstration
        let mut tasks = HashMap::new();
        let mut task_ids = Vec::new();
        
        for i in 1..20 {
            let id = format!("task-{}", i);
            let status = match i % 4 {
                0 => TaskStatus::Pending,
                1 => TaskStatus::Running,
                2 => TaskStatus::Completed,
                _ => TaskStatus::Failed,
            };
            
            let progress = match status {
                TaskStatus::Pending => 0.0,
                TaskStatus::Running => (i as f64 % 10.0) / 10.0,
                TaskStatus::Completed => 1.0,
                TaskStatus::Failed => (i as f64 % 10.0) / 10.0,
            };
            
            let task = Task {
                id: id.clone(),
                name: format!("Sample Task {}", i),
                status,
                progress,
                cpu_usage: (i as f64 % 100.0) / 100.0,
                memory_usage: (i as f64 % 80.0) / 100.0,
            };
            
            task_ids.push(id.clone());
            tasks.insert(id, task);
        }
        
        Self {
            tasks,
            selected_task_id: None,
            task_ids,
            should_quit: false,
            tab_index: 0,
        }
    }
}

impl App {
    /// Creates a new application with default state
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Handles key events
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
                true
            }
            KeyCode::Tab => {
                self.tab_index = (self.tab_index + 1) % 3; // Cycle through tabs
                false
            }
            KeyCode::BackTab => {
                self.tab_index = (self.tab_index + 2) % 3; // Cycle backwards
                false
            }
            KeyCode::Down => {
                self.next_task();
                false
            }
            KeyCode::Up => {
                self.previous_task();
                false
            }
            _ => false,
        }
    }
    
    /// Updates the application state
    pub fn update(&mut self) {
        // In a real implementation, this would fetch updated task information
        // For now, we'll just update the progress of running tasks
        for task in self.tasks.values_mut() {
            if task.status == TaskStatus::Running {
                task.progress += 0.01;
                if task.progress >= 1.0 {
                    task.progress = 1.0;
                    task.status = TaskStatus::Completed;
                }
            }
        }
    }
    
    /// Selects the next task in the list
    fn next_task(&mut self) {
        if self.task_ids.is_empty() {
            return;
        }
        
        let current_index = match &self.selected_task_id {
            Some(id) => self.task_ids.iter().position(|x| x == id).unwrap_or(0),
            None => 0,
        };
        
        let next_index = (current_index + 1) % self.task_ids.len();
        self.selected_task_id = Some(self.task_ids[next_index].clone());
    }
    
    /// Selects the previous task in the list
    fn previous_task(&mut self) {
        if self.task_ids.is_empty() {
            return;
        }
        
        let current_index = match &self.selected_task_id {
            Some(id) => self.task_ids.iter().position(|x| x == id).unwrap_or(0),
            None => 0,
        };
        
        let previous_index = if current_index == 0 {
            self.task_ids.len() - 1
        } else {
            current_index - 1
        };
        
        self.selected_task_id = Some(self.task_ids[previous_index].clone());
    }
}