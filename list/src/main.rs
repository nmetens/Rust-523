
const MAX_SIZE: u8 = 3; // Max size of list

struct Job {
    title: String,
    hourly: f32,
    applied: bool,
}

impl Job {
    // Constructor:
    pub fn new(title: String, hourly: f32) -> Self {
        Self {
            title,
            hourly,
            applied: false,
        } // Return self
    }

    // Display job info:
    pub fn display(&self) -> () {
        println!("Title: {}. Pay: {}. Applied: {}", 
        self.title, self.hourly, self.applied);
    }

    // Update application status:
    pub fn applied(&mut self,  applied: bool) {
        self.applied = applied;
    }
}

fn main() {

    let mut job1 = Job::new("Tutor".to_string(), 17.25);
    job1.applied(true);

    // Create a list of ints:
    let mut list: Vec<Job> = Vec::new();

    // Add items at the end of the list:
    list.push(job1);
    list.push(Job::new("Bus Driver".to_string(), 30.0));

    // Check the length of list:
    if list.len() > 5 {
        println!("Cannot exceed {} items.", MAX_SIZE);
    } else {
        // Traverse the list and print each item:
        for job in list {
            job.display(); 
        }
    }
}
