use structs::{maintarg::MaintArg, package::Package};

mod cli;
mod logic;
mod shell;
mod structs;
mod utils;

fn main() {
    let args = cli::args::Args::init();
    
    args.generate.iter().for_each(|p| {
        let p = MaintArg::new(p);
        let mut p = Package::new(p.repo, p.name).expect("Failed to form package");
        logic::r#gen::r#gen(&mut p).expect("Failed to generate package");
    });

    // Use MaintArg instead of Package because Package depends on BUILD, which doesn't yet exist
    args.add.iter().for_each(|p| {
        let p = MaintArg::new(p);
        logic::add::add(&p).expect("Failed to add package");

        // generate it after adding it
        let mut p = Package::new(p.repo, p.name).expect("Failed to form package");
        logic::r#gen::r#gen(&mut p).expect("Failed to generate package");
    });

    args.revise.iter().for_each(|p| {
        let p = MaintArg::new(p);
        let p = Package::new(p.repo, p.name).expect("Failed to form package");

        logic::rev::rev(&p).expect("Failed to revise package");

        // reform the package to update any new variables
        let mut p = Package::new(&p.repo, &p.name).expect("Failed to form package");
        logic::r#gen::r#gen(&mut p).expect("Failed to generate package");
    });

    args.update.iter().for_each(|p| {
        let new = MaintArg::new(p);
        let old = Package::new(new.repo, new.name).expect("Failed to form package");

        let vers = logic::upd::upd(&old, &new).expect("Failed to update package");
        let mut new = old;
        new.version = vers;

        // reform the package to update any new variables
        let mut p = Package::new(&new.repo, &new.name).expect("Failed to form package");
        logic::r#gen::r#gen(&mut p).expect("Failed to generate package");
    });

    args.remove.iter().for_each(|p| {
        let p = MaintArg::new(p);
        let p = Package::new(p.repo, p.name).expect("Failed to form package");
        logic::rm::rm(&p).expect("Failed to remove package");
    });

    if !args.r#move.is_empty() {
        let from = args.r#move.first().expect("Invalid syntax");
        let from = MaintArg::new(from);
        let from = Package::new(from.repo, from.name).expect("Failed to form package");

        let to = args.r#move.last().expect("Invalid syntax");
        let to = MaintArg::new(to);

        logic::r#move::r#move(&from, &to).expect("Failed to move package");
        let mut to = Package::new(to.repo, to.name).expect("Failed to form package");
        logic::r#gen::r#gen(&mut to).expect("Failed to generate package");
    }

    if !args.cp.is_empty() {
        let from = args.cp.first().expect("Invalid syntax");
        let from = MaintArg::new(from);
        let from = Package::new(from.repo, from.name).expect("Failed to form package");

        let to = args.cp.last().expect("Invalid syntax");
        let to = MaintArg::new(to);

        logic::cp::cp(&from, &to).expect("Failed to copy package");
        let mut to = Package::new(to.repo, to.name).expect("Failed to form package");
        logic::r#gen::r#gen(&mut to).expect("Failed to generate package");
    }

    if !args.alias.is_empty() {
        let origin = args.alias.first().expect("Invalid syntax");
        let origin = MaintArg::new(origin);
        let origin = Package::new(origin.repo, origin.name).expect("Failed to form package");

        let alias = args.alias.last().expect("Invalid syntax");
        let alias = MaintArg::new(alias);

        logic::alias::alias(&origin, &alias).expect("Failed to alias package");
        // shouldn't need regeneration
    }
}
