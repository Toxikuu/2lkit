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
        logic::r#gen::r#gen(&mut p.into()).expect("Failed to generate package");
    });

    // Use MaintArg instead of Package because Package depends on BUILD, which doesn't yet exist
    args.add.iter().for_each(|p| {
        let p = MaintArg::new(p);
        logic::add::add(&p).expect("Failed to add package");

        // generate it after adding it
        logic::r#gen::r#gen(&mut p.into()).expect("Failed to generate package");
    });

    args.revise.iter().for_each(|p| {
        let p: Package = MaintArg::new(p).into();
        logic::rev::rev(&p).expect("Failed to revise package");

        // reform the package to update any new variables and regenerate
        logic::r#gen::r#gen(&mut p.reform()).expect("Failed to generate package");
    });

    args.view.iter().for_each(|p| {
        let p: Package = MaintArg::new(p).into();
        logic::view::view(&p).expect("Failed to view package");
    });

    args.update.iter().for_each(|p| {
        let new = MaintArg::new(p);
        let old: Package = new.into();

        let vers = logic::upd::upd(&old, &new).expect("Failed to update package");
        let mut new = old;
        new.version = vers;

        // reform the package to update any new variables and regenerate
        logic::r#gen::r#gen(&mut new.reform()).expect("Failed to generate package");
    });

    args.remove.iter().for_each(|p| {
        let p: Package = MaintArg::new(p).into();
        logic::rm::rm(&p).expect("Failed to remove package");
    });

    if !args.r#move.is_empty() {
        let from = args.r#move.first().expect("Invalid syntax");
        let from: Package = MaintArg::new(from).into();

        let to = args.r#move.last().expect("Invalid syntax");
        let to = MaintArg::new(to);

        logic::r#move::r#move(&from, &to).expect("Failed to move package");
        logic::r#gen::r#gen(&mut to.into()).expect("Failed to generate package");
    }

    if !args.cp.is_empty() {
        let from = args.cp.first().expect("Invalid syntax");
        let from: Package = MaintArg::new(from).into();

        let to = args.cp.last().expect("Invalid syntax");
        let to = MaintArg::new(to);

        logic::cp::cp(&from, &to).expect("Failed to copy package");
        logic::r#gen::r#gen(&mut to.into()).expect("Failed to generate package");
    }

    if !args.alias.is_empty() {
        let origin = args.alias.first().expect("Invalid syntax");
        let origin: Package = MaintArg::new(origin).into();

        let alias = args.alias.last().expect("Invalid syntax");
        let alias = MaintArg::new(alias);

        logic::alias::alias(&origin, &alias).expect("Failed to alias package");
        // shouldn't need regeneration
    }

    if !args.restore.is_empty() {
        let p = args.restore.first().expect("Invalid syntax");
        let p = MaintArg::new(p);

        let commit = args.restore.last().expect("Invalid syntax");
        logic::restore::restore(&p, commit).expect("Failed to restore package");

        logic::r#gen::r#gen(&mut p.into()).expect("Failed to generate package");
    }
}
