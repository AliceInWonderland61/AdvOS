// src/mlfq.rs
// As we have read in the chapter, for MLFQ we have multiple queueus to mange processes. 
#[derive(Clone)]
//The task we're working with tha has an ID, Priority, the Remaining Time it needs to run for and the Total Executed Time 
//It's a single tasks that you want to run in the sysytem 
pub struct Process {
    pub id: u32, //The process ID; it's a unique number for that specific process like a tag number 
    pub priority: usize,  // Represents the current queue index; tells us the priority queue that process is currently in. 
    pub remaining_time: u32, //How much time the process still needs to finish. If the remaining time is 0, then the process is done
    pub total_executed_time: u32, // Keeps track of the total amount of time that specific process has run so far
}
// This is the manager for the Queues we're working with
//It contains a list of processes and it manages how they'll be running 
pub struct MLFQ {
    queues: Vec<Vec<Process>>, //Vector of queues; each queue holds a specific process
    num_levels: usize, //The number of priority levels we have (how many queues we have)
    time_quanta: Vec<u32>, //The timie limit for each queue
    current_time: u32, //The current time in the system
}

impl MLFQ {
    // new instance of the MLFW struct 
    // num_levels: how many priority levels or queues you want the MLFQ to have
    // time_quanta: vector that stores the time limits for each queue. 
    //Self just means to return an instace of MLFQ which refers to the MLFQ struct 
    pub fn new(num_levels: usize, time_quanta: Vec<u32>) -> Self {
        MLFQ {
            queues: vec![Vec::new(); num_levels], //create an empty queue for each priority level 
            num_levels, //stores the number of priority queues into the MLFQ struct 
            time_quanta, //stores the time limit for each queue 
            current_time: 0, //initializes the current time we've run to 0. It'll work like a clock 
        }
    }

    // Exercise 1: Queue Management
    pub fn add_process(&mut self, process: Process) {
        // TODO: Implement this function
        // Add the process to the appropriate queue based on its priority
        // Ensure the priority is within the valid range (0 to num_levels - 1)
        while process.priority>=self.num_levels{
            process.priority=self.num_levels-1;
            break;
        }
        self.queues[process.priority].push(process);
        
    }

    // Exercise 2: Process Execution
    pub fn execute_process(&mut self, queue_index: usize) {
        // TODO: Implement this function
        // Execute the process for its time quantum or until completion

        //check if we have a process in the queue. In MLFQ, the new incoming process takes priority
        if !self.queues[queue_index].is_empty(){
            //if it's not empty, then we pop the process we have in the queue so we can work on the new process
            
        }
        
        // Update remaining_time, total_executed_time, and current_time
        // Move the process to a lower priority queue if it doesn't complete
    }

    // Exercise 3: Priority Boost
    pub fn priority_boost(&mut self) {
        // TODO: Implement this function
        // Move all processes to the highest priority queue
        // Reset the priority of all processes to 0
    }

    // Simulate time passing and trigger a boost if needed
    pub fn update_time(&mut self, elapsed_time: u32) {
        self.current_time += elapsed_time;
        let boost_interval = 100;
        if self.current_time % boost_interval == 0 {
            self.priority_boost();
        }
    }
}

// Automated Test Cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        
        let process1 = Process { id: 1, priority: 0, remaining_time: 10, total_executed_time: 0 };
        let process2 = Process { id: 2, priority: 1, remaining_time: 5, total_executed_time: 0 };
        let process3 = Process { id: 3, priority: 5, remaining_time: 8, total_executed_time: 0 };

        mlfq.add_process(process1);
        mlfq.add_process(process2);
        mlfq.add_process(process3);

        assert_eq!(mlfq.queues[0].len(), 1);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[2].len(), 1);
    }

    #[test]
    fn test_execute_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[0].push(Process { id: 1, priority: 0, remaining_time: 5, total_executed_time: 0 });

        mlfq.execute_process(0);

        assert_eq!(mlfq.queues[0].len(), 0);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[1][0].remaining_time, 3);
        assert_eq!(mlfq.queues[1][0].total_executed_time, 2);
    }

    #[test]
    fn test_priority_boost() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });
        mlfq.queues[2].push(Process { id: 2, priority: 2, remaining_time: 3, total_executed_time: 7 });

        mlfq.update_time(100); // Should trigger priority boost

        assert_eq!(mlfq.queues[0].len(), 2);
        assert_eq!(mlfq.queues[1].len(), 0);
        assert_eq!(mlfq.queues[2].len(), 0);
    }

    #[test]
    fn test_boost_does_not_occur_prematurely() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });
        
        mlfq.update_time(50); // No boost should happen

        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[0].len(), 0);
    }
}