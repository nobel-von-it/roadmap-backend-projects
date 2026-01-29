use github_activity::{
    error::Error,
    event::{get_events, ControledEvents, Event},
};

fn run(args: &[String]) -> Result<(), Error> {
    if args.is_empty() {
        return Err(Error::ArgumentError(
            "Please provide a GitHub username".to_string(),
        ));
    }
    let username = &args[0];
    let events = get_events(username)?
        .iter()
        .filter(|je| je.public)
        .filter(|je| je.payload.payload_ref.is_some())
        .map(Event::from)
        .collect::<Vec<Event>>();

    let controled_events = ControledEvents::from(events.as_slice());

    println!("{}", controled_events);

    Ok(())
}

fn main() -> Result<(), Error> {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    if let Err(e) = run(&args) {
        eprintln!("{}", e);
    }
    Ok(())
}
