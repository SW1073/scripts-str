use scheduler::
{
    schedulers::{
    cyclic::CyclicScheduler,
    monotonic::{
        deadline::DeadlineMonotonicScheduler,
        rate::RateMonotonicScheduler,
    },
    edf::EarliestDeadlineFirstScheduler,
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
    let mut sched = EarliestDeadlineFirstScheduler::new();
    sched.add_task(10.0, 20, 30).unwrap();
    sched.add_task(20.0, 55, 70).unwrap();
    sched.add_task(20.0, 60, 100).unwrap();

    print_is_schedulable(&mut sched);

}
