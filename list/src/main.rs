mod job; // References job.rs file
mod application; // References application.rs file

fn main() {
    let mut apps = application::Applications::new();
    apps.add_job();
    apps.view_apps();
}
