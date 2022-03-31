// use crate::matrix;
use crate::common;
use std::cmp::{Ord, Ordering};
use std::collections::binary_heap;
use std::ops::Add;
use std::sync::mpsc;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub enum Command {
    MatrixCommand(common::MatrixCommand),
}

pub struct DelayedCommand {
    command: Command,
    delay: Option<Duration>,
}

impl DelayedCommand {
    pub fn new(command: Command, delay: Option<Duration>) -> DelayedCommand {
        DelayedCommand { command, delay }
    }
}

struct ScheduledCommand {
    command: Command,
    scheduled_time: Instant,
}

impl ScheduledCommand {
    fn new(command: Command, delay: Duration) -> ScheduledCommand {
        ScheduledCommand {
            command,
            scheduled_time: Instant::now().add(delay),
        }
    }
}
impl Ord for ScheduledCommand {
    fn cmp(&self, other: &Self) -> Ordering {
        other.scheduled_time.cmp(&self.scheduled_time)
    }
}

impl PartialOrd for ScheduledCommand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for ScheduledCommand {}

impl PartialEq for ScheduledCommand {
    fn eq(&self, other: &Self) -> bool {
        self.scheduled_time == other.scheduled_time
    }
}

pub struct Scheduler {
    receiver: mpsc::Receiver<DelayedCommand>,
    queue: binary_heap::BinaryHeap<ScheduledCommand>,
    matrix_sender: mpsc::Sender<common::MatrixCommand>,
}

impl Scheduler {
    pub fn new(
        receiver: mpsc::Receiver<DelayedCommand>,
        matrix_sender: mpsc::Sender<common::MatrixCommand>,
    ) -> Scheduler {
        Scheduler {
            receiver,
            queue: binary_heap::BinaryHeap::new(),
            matrix_sender,
        }
    }

    pub fn run(&mut self) {
        loop {
            //TODO use two threads here--one to read and instantly process/queue, the other to periodically pull off the queue
            if let Ok(delayed_command) = self.receiver.try_recv() {
                // queue the command
                match delayed_command.delay {
                    Some(delay) => {
                        self.queue
                            .push(ScheduledCommand::new(delayed_command.command, delay));
                    }
                    None => self.send_command(delayed_command.command),
                }
            }
            let now = Instant::now();
            match self.queue.pop() {
                Some(scheduled_command) => {
                    if scheduled_command.scheduled_time < now {
                        self.send_command(scheduled_command.command);
                    } else {
                        // Delay a little bit
                        self.queue.push(scheduled_command);
                        sleep(Duration::from_millis(5));
                    }
                }
                None => {
                    sleep(Duration::from_millis(5));
                }
            }
        }
    }

    fn send_command(&self, command: Command) {
        match command {
            Command::MatrixCommand(matrix_command) => {
                self.matrix_sender.send(matrix_command).unwrap();
            }
        }
    }
}
