// Created by Aaron C Gaudette on 25.08.17

extern crate i3ipc;
use i3ipc::I3Connection;
use i3ipc::I3EventListener;
use i3ipc::Subscription;
use i3ipc::event::Event;
use i3ipc::event::inner;

use std::env;
use std::thread;
use std::time::Duration;

fn parse(args: &[String]) -> i32 {
  if args.len() == 1 {
    panic!("Not enough arguments!");
  }

  match args[1].parse::<i32>() {
    Ok(i) => { i },
    Err(e) => {
      panic!("Incorrect argument: {}", e);
    }
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let highlight = parse(&args);

  let millis = 1000;

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
          // On new window focus
          inner::WindowChange::Focus => {
            // Ignore quake console
            if e.container.name.unwrap() != "quaketerm" {
              // Clear all borders
              i3.command("[tiling] border none").unwrap();

              // Highlight
              let id = e.container.id;
              i3.command(
                &format!("[con_id=\"{}\"] border pixel {}", id, highlight)
              ).unwrap();
              //i3.command(&format!("border pixel {}", highlight)).unwrap();

              deselect(id, millis);
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

fn deselect(id: i32, millis: u64) {
  thread::spawn(move || {
    let mut i3 = I3Connection::connect().ok().expect("Failed to connect");
    thread::sleep(Duration::from_millis(millis));
    i3.command(&format!("[con_id=\"{}\"] border none", id)).unwrap();
  });
}
