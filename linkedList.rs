use std::collections::LinkedList;
use std::io::{self, Write};

struct Room {
    description: &'static str,
}

fn main() {
    let mut dungeon: LinkedList<Room> = LinkedList::new();

    dungeon.push_back(Room { description: "You are in a dark room. There is a door to the east." });
    dungeon.push_back(Room { description: "You are in a brightly lit room. There is a door to the east and west." });
    dungeon.push_back(Room { description: "You are in a cold room. There is a door to the west." });

    let mut current_room = dungeon.front();

    loop {
        match current_room {
            Some(room) => {
                println!("{}", room.description);
                print!("Command: ");
                io::stdout().flush().unwrap();

                let mut command = String::new();
                io::stdin().read_line(&mut command).unwrap();

                match command.trim() {
                    "east" => {
                        if let Some(next_room) = dungeon.split_off(1).front() {
                            current_room = Some(next_room);
                        } else {
                            println!("There is no door in that direction.");
                        }
                    },
                    "west" => {
                        if let Some(next_room) = dungeon.split_off(dungeon.len() - 1).front() {
                            current_room = Some(next_room);
                        } else {
                            println!("There is no door in that direction.");
                        }
                    },
                    "quit" => break,
                    _ => println!("Invalid command."),
                }
            },
            None => panic!("You have fallen into the void."),
        }
    }
}
