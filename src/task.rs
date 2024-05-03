use std::fmt;

use crate::game_state::*;

pub const HEAD_SENTINEL: usize = 0xFE;
pub const TAIL_SENTINEL: usize = 0xFF;
pub const TASK_NONE: usize = TAIL_SENTINEL;

pub const NUM_TASKS: usize = 16;
pub const NUM_TASK_DATA: usize = 16;
pub const FOLLOW_UP_INDEX: usize = NUM_TASK_DATA - 2;

pub trait TaskFunc {
    fn call(&mut self, state: &mut GameState, task_id: usize);
}

impl<F> TaskFunc for F
where
    F: FnMut(&mut GameState, usize) + 'static,
{
    fn call(&mut self, state: &mut GameState, task_id: usize) {
        self(state, task_id)
    }
}

pub struct Task {
    task_function: Option<Box<dyn TaskFunc>>,
    followup_function: Option<Box<dyn TaskFunc>>,
    is_active: bool,
    prev: usize,
    next: usize,
    priority: i32,
    data: [i16; NUM_TASK_DATA],
}

impl fmt::Debug for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Task")
            .field("is_active", &self.is_active)
            .field("prev", &self.prev)
            .field("next", &self.next)
            .field("priority", &self.priority)
            .field("data", &self.data)
            .finish()
    }
}

impl Task {
    pub fn new(index: usize) -> Self {
        Self {
            task_function: Some(Box::new(task_dummy)),
            followup_function: None,
            is_active: false,
            prev: index,
            next: index + 1,
            priority: -1,
            data: [0; NUM_TASK_DATA],
        }
    }

    pub fn set_function<F: TaskFunc + 'static>(&mut self, func: F) {
        self.task_function = Some(Box::new(func))
    }
}

pub struct TaskList {
    tasks: [Task; NUM_TASKS],
}

impl TaskList {
    pub fn new() -> Self {
        let mut tasks: Vec<Task> = Vec::new();
        for i in 0..NUM_TASKS {
            let new_task = Task::new(i);
            tasks.push(new_task);
        }

        let task_array: [Task; NUM_TASKS] = tasks.try_into().unwrap();

        Self { tasks: task_array }
    }

    pub fn reset_tasks(&mut self) -> () {
        for i in 0..NUM_TASKS {
            self.tasks[i] = Task::new(i);
        }

        self.tasks[0].prev = HEAD_SENTINEL;
        self.tasks[NUM_TASKS - 1].next = TAIL_SENTINEL;
    }

    pub fn create_task<F: TaskFunc + 'static>(&mut self, func: F, priority: i32) -> usize {
        for i in 0..NUM_TASKS {
            let task = &mut self.tasks[i];
            if !task.is_active {
                task.set_function(func);
                task.priority = priority;
                task.data = [0; NUM_TASK_DATA];
                task.is_active = true;
                self.insert_task(i);
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
            if self.tasks[new_task_id].priority < self.tasks[task_id].priority {
                // insert the new task before the task with higher priority
                self.tasks[new_task_id].prev = self.tasks[task_id].prev;
                self.tasks[new_task_id].next = task_id;
                if self.tasks[task_id].prev != HEAD_SENTINEL {
                    self.tasks[self.tasks[task_id].prev].next = new_task_id;
                }
                self.tasks[task_id].next = new_task_id;
                return;
            }
            if self.tasks[task_id].next == TAIL_SENTINEL {
                // we are at the end
                self.tasks[new_task_id].prev = task_id;
                self.tasks[new_task_id].next = self.tasks[task_id].next;
                self.tasks[task_id].next = new_task_id;
                return;
            }
            task_id = self.tasks[task_id].next;
        }
    }

    pub fn destroy_task(&mut self, task_id: usize) {
        if self.tasks[task_id].is_active {
            self.tasks[task_id].is_active = false;

            if self.tasks[task_id].prev == HEAD_SENTINEL {
                if self.tasks[task_id].next != TAIL_SENTINEL {
                    self.tasks[self.tasks[task_id].next].prev = HEAD_SENTINEL;
                }
            } else {
                if self.tasks[task_id].next == TAIL_SENTINEL {
                    self.tasks[self.tasks[task_id].prev].next = TAIL_SENTINEL;
                } else {
                    self.tasks[self.tasks[task_id].prev].next = self.tasks[task_id].next;
                    self.tasks[self.tasks[task_id].next].prev = self.tasks[task_id].prev;
                }
            }
        }
    }

    pub fn run_tasks(&mut self, g: &mut GameState) -> () {
        let mut task_id: usize = self.find_first_active_task();

        if task_id != NUM_TASKS {
            loop {
                let current_task = &mut self.tasks[task_id];

                // (current_task.task_function).call(g, task_id);
                match &mut current_task.task_function {
                    Some(f) => f.call(g, task_id),
                    _ => panic!(""),
                }

                task_id = current_task.next;

                if task_id == TAIL_SENTINEL {
                    break;
                }
            }
        }
    }

    pub fn find_first_active_task(&self) -> usize {
        let mut task_id: usize = 0;
        for i in 0..self.tasks.len() {
            if self.tasks[task_id].is_active && self.tasks[task_id].prev == HEAD_SENTINEL {
                task_id = i;
                break;
            }
        }
        task_id
    }

    pub fn set_task_func_with_followup(
        &mut self,
        task_id: usize,
        func: Box<dyn TaskFunc>,
        followup_func: Box<dyn TaskFunc>,
    ) {
        self.tasks[task_id].followup_function = Some(followup_func);
        self.tasks[task_id].task_function = Some(func);
    }

    pub fn switch_task_to_followup(&mut self, task_id: usize) -> () {
        self.tasks[task_id].task_function = self.tasks[task_id].followup_function.take()
    }

    // pub fn function_is_active_task(&self, func: Box<dyn TaskFunc>) {
    //     for i in 0..NUM_TASKS {
    //         if self.tasks[i].is_active == true && self.tasks[i].task_function  {}
    //     }
    // }

    pub fn num_active_tasks(&self) -> u8 {
        let mut count = 0;

        for i in 0..NUM_TASKS {
            if self.tasks[i].is_active == true {
                count += 1;
            }
        }

        count
    }

    pub fn set_word_task_arg(&mut self, task_id: usize, data_elem: usize, value: u32) {
        if data_elem < NUM_TASK_DATA - 1 {
            self.tasks[task_id].data[data_elem] = value as i16;
            self.tasks[task_id].data[data_elem + 1] = (value >> 16) as i16;
        }
    }

    pub fn get_word_task_arg(&self, task_id: usize, data_elem: usize) -> u32 {
        if data_elem < NUM_TASK_DATA - 1 {
            return (self.tasks[task_id].data[data_elem] as i16
                | (self.tasks[task_id].data[data_elem] << 16)) as u32;
        }
        0
    }
}

fn task_dummy(g: &mut GameState, task_id: usize) -> () {
    ()
}
