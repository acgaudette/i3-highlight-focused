// Created by Aaron C Gaudette on 25.08.17

use std::env;
use std::thread;
use std::time::Duration;

extern crate i3ipc;
use i3ipc::I3Connection;
use i3ipc::I3EventListener;
use i3ipc::Subscription;
use i3ipc::event::Event;
use i3ipc::event::inner;

fn main() {
  let args: Vec<String> = env::args().collect();
  let (highlight, millis) = parse(&args);

  let mut i3 = I3Connection::connect().ok().expect("Failed to connect");
  let mut listener = I3EventListener::connect().ok().expect("Failed to bind");

  let subs = [ Subscription::Window ];
  listener.subscribe(&subs).ok().expect("Failed to subscribe");

  for ev in listener.listen() {
    match ev {
      Ok(Event::WindowEvent(e)) => {
        match e.change {
          // On new window focus
          inner::WindowChange::Focus => {
            // Ignore quake console
            if e.container.name.unwrap() == "quaketerm" { continue; }

            // Ignore stacked windows
            if e.container.deco_rect.2 != 0 { continue; }

            // Clear all borders
            i3.command("[tiling] border none").unwrap();

            // Highlight
            let id = e.container.id;
            i3.command(
              &format!("[con_id=\"{}\"] border pixel {}", id, highlight)
            ).unwrap();

            // Wait and remove highlight
            deselect(id, millis);
          },
          _ => { }
        }
      },

      Err(e) => panic!("Error: {}", e),
      _ => unreachable!()
    }
  }
}

fn parse(args: &[String]) -> (i32, u64) {
  if args.len() < 2 {
    panic!("Not enough arguments!");
  }

  let hi = match args[1].parse::<i32>() {
    Ok(i) => i, Err(e) => {
      panic!("Incorrect argument: {}", e);
    }
  };

  let m = match args[2].parse::<u64>() {
    Ok(i) => i, Err(e) => {
      panic!("Incorrect argument: {}", e);
    }
  };

  (hi, m)
}

fn deselect(id: i32, millis: u64) {
  thread::spawn(move || {
    let mut i3 = I3Connection::connect().ok()
      .expect("Failed to connect (aux thread)");

    thread::sleep(Duration::from_millis(millis));
    // Remove highlight
    i3.command(&format!("[con_id=\"{}\"] border none", id)).unwrap();
  });
}
