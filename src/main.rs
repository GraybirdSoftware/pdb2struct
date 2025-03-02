use std::{io::Write, path::PathBuf};
use clap::{Parser, ValueEnum};


mod hpp;


#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum OutputFormat { 
    Rust,
    Hpp,
}


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    #[arg(short, long)]
    pdb: PathBuf,

    #[arg(short, long, value_enum, default_value_t = OutputFormat::Hpp)]
    format: OutputFormat, 

    #[arg(short, long)]
    out: Option<PathBuf>,
}


fn main() -> pdb::Result<()>{
    color_eyre::install().unwrap();

    env_logger::init();

    let args = Args::parse(); 
    
    if !args.pdb.exists() { 
        println!("pdb file cannot be found");
        std::process::exit(1);
    }


    let file = std::fs::File::open(args.pdb)?;
    let mut pdb: pdb::PDB<'_, std::fs::File> = pdb::PDB::open(file)?;




    let output = match args.format { 
        OutputFormat::Rust => { 
            todo!();
        },
        OutputFormat::Hpp => { 
            hpp::hpp_main(&mut pdb)
        }
    }?;


    if args.out == None { 
        spit_stdout(output.as_str());
    }

    Ok(())
}


fn spit_stdout(content: &str) { 
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    handle.write_all(content.as_bytes());
}






