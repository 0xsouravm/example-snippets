use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo", about = "A simple todo CLI", version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add to the todo
    Add {
        listname: String,
        item: String,
    },
    /// Show the items
    Show {
        listname: Option<String>,
        #[command(subcommand)]
        showcommand: Option<ShowCommands>,
    },
    /// Login into the todo service
    Login,
    /// Logout from the todo service
    Logout,
}

#[derive(Subcommand)]
enum ShowCommands {
    /// Complete
    Complete,
    /// Incomplete
    Incomplete, 
    /// All
    All
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Add { listname, item }) => {
            println!("Adding item {} to list {}", item, listname);
        }
        Some(Commands::Logout) => {
            println!("Logging out");
        }
        Some(Commands::Login) => {
            println!("Logging in");
        }
        Some(Commands::Show { listname, showcommand }) => {
            match showcommand {
                Some(ShowCommands::All) => {
                    match listname {
                        Some(listname) => {
                            println!("Showing all items of list {}", listname);
                        }
                        None => {
                            println!("Showing all the list names");
                        }
                    }
                }
                Some(ShowCommands::Complete) => {
                    match listname {
                        Some(listname) => {
                            println!("Showing all completed items of list {}", listname);
                        }
                        None => {
                            println!("Showing all completed items of all lists");
                        }
                    }
                }
                Some(ShowCommands::Incomplete) => {
                    match listname {
                        Some(listname) => {
                            println!("Showing all incomplete items of list {}", listname);
                        }
                        None => {
                            println!("Showing all incomplete items of all lists");
                        }
                    }
                }
                None => {
                    match listname {
                        Some(listname) => {
                            println!("Showing all items of list {}", listname);
                        }
                        None => {
                            println!("Showing all the list names");
                        }
                    }
                }
            }
        }
        None => {
            println!("No command provided");
        }
    }
}

// todo show - Shows all the list names.
// todo show -a - Shows all the list names along with the items.
// todo show -c - Shows all the completed items of all lists
// todo show -i - Shows all the incomplete items of all lists
// todo show <list_name> -c - Shows all the completed items of that list
// todo show <list_name> -i - Shows all the incompleted items of that list
// todo show <list_name> -a - Shows all the items of that list

// todo add <list_name> <item> - Adds the item to that list

// todo complete <list_name> <item_number> - Marks that particular item of that list as completed
// todo complete <list_name> - Marks all the items of that list as completed
// todo complete - Marks all items of all lists as completed

// todo incomplete <list_name> <item_number> - Marks that particular item of that list as incomplete

// todo remove - removes all lists
// todo remove complete - removes all complete items from all lists
// todo remove incomplete - removes all incomplete items from all lists
// todo remove <list_name> - removes that particular list
// todo remove <list_name> -c - removes all the complete items in that particular list
// todo remove <list_name> -i - removes removes all the incomplete items in that particular list
// todo remove <list_name> <item_number> - removes that item from the list

// // Optional
// todo login - optional. Prompted for email and password. Creates new user if email is valid and doesn't exist before.
// todo logout - logs out the user if any logged in
// todo push - Sync with cloud
// todo pull - Sync with cloud

// // Optional
// todo share list_name email - Shares the list with the user with that email so they contribute together
// todo unshare list_name email - Unshares the list with the user with that email
// todo show shared - Shows all the shared lists
// todo show shared --list <list_name> - Shows all the users with whom the list is shared
// todo show shared --email <email> - Shows all the items of that list shared with that user
// todo show notifications -u - Shows all the unread notifications of the user( this is where the todo share request comes in. If you accept then the list is shared with you)
// todo show notifications -r - Shows all the read notifications of the user
// todo show notifications - Shows all the notifications of the user