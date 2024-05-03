use crate::game_state::*;

pub const HEAD_SENTINEL: usize = 0xFE;
pub const TAIL_SENTINEL: usize = 0xFF;
pub const TASK_NONE: usize = TAIL_SENTINEL;

pub const NUM_TASKS: usize = 16;
pub const NUM_TASK_DATA: usize = 16;

pub struct Task {
    task_function: Box<dyn FnMut(&mut GameState, usize)>,
    is_active: bool,
    prev: usize,
    next: usize,
    priority: i32,
    data: [i16; NUM_TASK_DATA],
}

impl Task {
    pub fn new(index: usize) -> Self {
        Self {
            task_function: Box::new(task_dummy),
            is_active: false,
            prev: index,
            next: index + 1,
            priority: -1,
            data: [0; NUM_TASK_DATA],
        }
    }

    pub fn set_function<F>(&mut self, func: F)
    where
        F: FnMut(&mut GameState, usize) + 'static,
    {
        self.task_function = Box::new(func)
    }
}

pub struct TaskList {
    tasks: [Task; NUM_TASKS],
}

impl TaskList {
    pub fn reset_tasks(&mut self) -> () {
        for i in 0..NUM_TASKS {
            self.tasks[i] = Task::new(i);
        }

        self.tasks[0].prev = HEAD_SENTINEL;
        self.tasks[NUM_TASKS - 1].next = TAIL_SENTINEL;
    }

    pub fn create_task<F>(&mut self, func: F, priority: i32) -> usize
    where
        F: FnMut(&mut GameState, usize) + 'static,
    {
        for i in 0..NUM_TASKS {
            let task = &mut self.tasks[i];
            if !task.is_active {
                task.set_function(func);
                task.priority = priority;
                self.insert_task(i);
                task.data = [0; NUM_TASK_DATA];
                task.is_active = true;
                return i;
            }
        }

        0
    }

    pub fn insert_task(&mut self, new_task_id: usize) -> () {
        let mut task_id: usize = self.find_first_active_task();

        if task_id == NUM_TASKS {
            // the new task is the only task
            self.tasks[new_task_id].prev = HEAD_SENTINEL;
            self.tasks[new_task_id].next = TAIL_SENTINEL;
            return;
        }

        loop {
            let active_task = &mut self.tasks[task_id];
            let new_task = &mut self.tasks[new_task_id];

            if new_task.priority < active_task.priority {
                // insert the new task before the task with higher priority
                new_task.prev = active_task.prev;
                new_task.next = task_id;
                if active_task.prev != HEAD_SENTINEL {
                    self.tasks[active_task.prev].next = new_task_id;
                }
                active_task.next = new_task_id;
                return;
            }
            if active_task.next == TAIL_SENTINEL {
                // we are at the end
                new_task.prev = task_id;
                new_task.next = active_task.next;
                active_task.next = new_task_id;
                return;
            }
            task_id = active_task.next;
        }
    }

    pub fn destroy_task(&mut self, task_id: usize) {
        let task = &mut self.tasks[task_id];
        if task.is_active {
            task.is_active = false;

            if task.prev == HEAD_SENTINEL {
                if task.next != TAIL_SENTINEL {
                    self.tasks[task.next].prev = HEAD_SENTINEL;
                }
            } else {
                if task.next == TAIL_SENTINEL {
                    self.tasks[task.prev].next = TAIL_SENTINEL;
                } else {
                    self.tasks[task.prev].next = task.next;
                    self.tasks[task.next].prev = task.prev;
                }
            }
        }
    }

    pub fn run_tasks(&self, g: &mut GameState) -> () {
        let mut task_id: usize = self.find_first_active_task();

        if task_id != NUM_TASKS {
            loop {
                let current_task = &mut self.tasks[task_id];
                (current_task.task_function)(g, task_id);
                task_id = current_task.next;

                if task_id == TAIL_SENTINEL {
                    break;
                }
            }
        }
    }

    pub fn find_first_active_task(&self) -> usize {
        let mut task_id: usize;
        for task_id in 0..self.tasks.len() {
            let task = self.tasks[task_id];
            if task.is_active && task.prev == HEAD_SENTINEL {
                break;
            }
        }
        task_id
    }
}

fn task_dummy(g: &mut GameState, task_id: usize) -> () {
    ()
}
