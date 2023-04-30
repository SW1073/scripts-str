use scheduler::
{
    schedulers::{
    cyclic::CyclicScheduler,
    monotonic::{
        deadline::DeadlineMonotonicScheduler,
        rate::RateMonotonicScheduler,
    },
    SchedulabilityResult::{
        Schedulable as Schedulable,
        NotSchedulable as NotSchedulable,
        Undetermined as Undetermined,
    },
    CheckSchedulable
},
log::Log,
};

fn print_if_log(log: Option<Log>) {
    match log {
        Some(l) => {println!("Log:"); l.print_log()},
        None => println!("No s'ha proporcionat Log."),
    }
}

fn print_is_schedulable(sched: &mut dyn CheckSchedulable) {
    match sched.is_schedulable() {
        Schedulable(log) => {
            println!("El sistema SI es planificable");
            print_if_log(log);
        },
        NotSchedulable(log) => {
            println!("El sistema NO es planificable.");
            print_if_log(log);
        },
        Undetermined(log) => {
            println!("No s'ha pogut detrminar si el sistema es planificable.");
            print_if_log(log);
        }
    };
}

fn main() {
    // Input Scheduler Type

    // Input Number of Tasks

    // Check schedulability
    let mut sched = CyclicScheduler::new();
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(1.0, 10, 10).unwrap();
    sched.add_task(6.0, 40, 40).unwrap();

    print_is_schedulable(&mut sched);

}
