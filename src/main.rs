use colored::*;
use exitfailure::ExitFailure;
use failure::ResultExt;
use std::io::{self, Read};
use structopt::StructOpt;

const CAT: &str = "
,.                 .,
,: ':.    .,.    .:' :,
,',   '.:'   ':.'   ,',
: '.  '         '  .' :
', : '           ' : ,'
'.' .,:,.   .,:,. '.'
 ,:    x '. .' x    :,
,:        / '        :,
,:                   :,
 ,:       =:=       :,
  ,: ,     :     , :,
   :' ',.,' ',.,:'':
 :'      ':WW::'   '.
.:'       '::::'   ':
,:        '::::'     :,";

const FULLCAT: &str = "

,.                 .,
,: ':.    .,.    .:' :,
,',   '.:'   ':.'   ,',
: '.  '         '  .' :
', : '           ' : ,'
'.' .,:,.   .,:,. '.'
 ,:    V '. .' V    :,
,:        / '        :,
,:                   :,
 ,:       =:=       :,
  ,: ,     :     , :,
   :' ',.,' ',.,:' ':
  :'      ':WW::'   '.
 .:'       '::::'   ':
 ,:        '::::'    :,
 :'         ':::'    ':
,:           ':''     :.
.:'             '.     ',.
,:'               ''     '.
.:'               .',    ':
.:'               .'.,     :
.:                .,''     :
::                .,''    ,:
::              .,'','   .:'
.,::'.           .,','     ::::.
.:'     ',.       ,:,       ,WWWWW,
:'        :       :W:'     :WWWWWWW,          .,.
:         ',      WWW      WWWWWWWWW          '::,
'.         ',     WWW     :WWWWWWWWW            '::,
'.         :     WWW     :WWWWWWWW'             :::
'.       ,:     WWW     :WWWWWWW'             .:::
'.     .W:     WWW     :WWWWWW'           .,:::'
'.   :WW:     WWW     :WWWWW'      .,,:::::''
.,'   ''::     :W:     :WWWWW.  .,::::''
,'        ''','',',','','''WWWWW::::''
':,,,,,,,':  :  : : :  :  :WWWW'''";

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Created by Taufiq Rahman")]
    /// meow?
    message: String,

    #[structopt(short = "b", long = "full")]
    /// Full view
    full_view: bool,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Use your own ASCII cat from file
    file: Option<std::path::PathBuf>,

    #[structopt(short = "i", long = "stdin")]
    /// Read the message from stdin
    stdin: bool,
}

fn main() -> Result<(), ExitFailure> {
    let options = Options::from_args();
    let mut message = "".to_string();

    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
        message.pop();
    } else {
        message = options.message;
    }

    if options.full_view {
        println!("{}", FULLCAT.to_string().red());
    } else {
        match &options.file {
            Some(path) => {
                let alt = std::fs::read_to_string(path)
                    .with_context(|_| format!("Could not read file {}", path.display()))?;
                print_message(message);
                println!("{}", alt.to_string().bright_yellow());
            }
            None => {
                print_message(message);
                println!("{}", CAT.to_string().bright_yellow());
            }
        }
    }

    Ok(())
}

fn print_message(message: String) {
    if message.to_lowercase().contains("woof") {
        eprintln!("Woah, friend! No woof woof!");
    } else {
        println!(" {}", "-".repeat(message.chars().count() + 2));
        println!("| {} |", message);
        println!(" {}", "-".repeat(message.chars().count() + 2));
        println!("  \\  /");
        println!("   \\/");
    }
}
