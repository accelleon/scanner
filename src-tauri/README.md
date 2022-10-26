Rust backend supporting the scanning interface

Flow for jobs should be as follows
* Tauri invokes task
* Invoked rust function sets the current working job state and runs the task
* Task should asynchronously report progress and results to the frontend
* Invoked rust function should not complete before the task is complete
