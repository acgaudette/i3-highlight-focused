extern crate i3ipc;
use i3ipc::I3Connection;
use i3ipc::I3EventListener;
use i3ipc::Subscription;
use i3ipc::event::Event;
use i3ipc::event::inner;

fn main() {
  let mut i3 = I3Connection::connect().ok().expect("Failed to connect");
  let mut listener = I3EventListener::connect().ok().expect("Failed to bind");

  let subs = [
    Subscription::Window,
  ];

  // Subscribe
  listener.subscribe(&subs).ok().expect("Failed to subscribe");

  for ev in listener.listen() {
    match ev {
      Ok(Event::WindowEvent(e)) => {
        match e.change {
          inner::WindowChange::Focus => {
            if e.container.name.unwrap() != "quaketerm" {
              i3.command("[tiling] border none").unwrap();
              i3.command("border pixel 8").unwrap();
            }
          }
          _ => { }
        }
      },
      Err(e) => println!("Error: {}", e),
      _ => unreachable!()
    }
    //println!("{:?}\n", ev.ok())
  }
}
