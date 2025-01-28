
struct Job {
    job_id: u32,
    title: String,
    hourly: f32,
    applied: bool,
}

impl Job {
    // Constructor:
    pub fn new(job_id: u32, title: String, hourly: f32) -> Self {
        Self {
            job_id,
            title,
            hourly,
            applied: false,
        } // Return self
    }

    // Display job info:
    pub fn display(&self) -> () {
        println!("Job ID: {}. Title: {}. Pay: {}. Applied: {}", 
        self.job_id, self.title, self.hourly, self.applied);
    }

    // Update application status:
    pub fn applied(&mut self,  applied: bool) {
        self.applied = applied;
    }
}

// Holds a list of jobs:
struct Applications {
    jobs: Vec<Job>,
    total_jobs: u32, // How many jobs are in the app
}

impl Applications {
    // Construct a new application:
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
            total_jobs: 0,
        }
    }

    pub fn add_job(&mut self, job: Job) {
        self.jobs.push(job);
        self.total_jobs += 1;
    }

    pub fn view_apps(self) {
        for job in self.jobs {
            job.display();
        }
    }
}

fn main() {

    let mut apps = Applications::new();
    
    let mut job1 = Job::new(1, "Bus Driver".to_string(), 30.0);
    apps.add_job(job1);
    apps.add_job(Job::new(2, "Tutor".to_string(), 17.25));

    apps.view_apps();
}
