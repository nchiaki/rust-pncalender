use std::env;
//use std::fmt::Display;
use std::path::Path;
use std::ffi::OsStr;

mod help;
mod prime;
mod output;
use crate::help::setopt;

fn main() {
    //println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let argc = args.len();

    let cmdnm = match Path::new(&args[0]).file_name()
    {
        Some(v) => v,
        None => {
            OsStr::new("Bye bye ...");
            return;
        },
    };

    setopt::init();
    output::init();

    //println!("argc:{}", argc);
    if 2 <= argc
    {
        let mut ix = 1;
        loop
        {
            let mut arg = &args[ix];
            //println!("{}:{}", ix, arg);
    /***
    #argp.add_argument('-mlt', '--multi', action='store_true', help='並列処理を有効にする')
    argp.add_argument('-mlt', '--multi', choices=['thrd','prcs','prcspl','thrdpl','none'], default='none', help='並列処理を有効にする')
    argp.add_argument('-cpu', '--cpu', type=int, default=1, help='コア数')
    argp.add_argument('-dly', '--dlycoefficient', type=int, default=0, help='処理遅延係数（0:=遅延なし）')
    ***/
            if arg == "-h" || arg == "--help" || arg == "?"
            {
                help::usage(cmdnm);
                return;
            }
            else if arg == "-of" || arg == "--outformat"
            {
                ix += 1;
                if argc <= ix
                {
                    help::undecided_parameter_msg(arg);
                    help::usage(cmdnm);
                    return;
                }
                arg = &args[ix];
                if arg == "nrml" || arg == "bnnr" || arg == "none"
                {setopt::outformat(String::from(arg));}
                else
                {
                    help::illegal_param_msg(arg);
                    help::usage(cmdnm);
                    return;
                }
            }
            else if arg == "-prm" || arg == "--prime"
            {
                ix += 1;
                if argc <= ix
                {
                    help::undecided_parameter_msg(arg);
                    help::usage(cmdnm);
                    return;
                }
                /* 開始値確認 */
                arg = &args[ix];
                match arg.parse::<u32>()
                {
                    Ok(v) => setopt::primestart(v),
                    Err(_) => {
                        help::illegal_param_msg(arg);
                        help::usage(cmdnm);
                        return;
                    },
                };
                ix += 1;
                if argc <= ix
                {/*正常終了*/
                    break;
                }
                /* 終了値確認 */
                arg = &args[ix];
                match arg.parse::<u32>()
                {
                    Ok(v) => setopt::primeend(v),
                    Err(_) => {
                        if arg.starts_with("-")
                        {
                            /* 次のパラメータ解析に回す */
                            ix -= 1;
                        }
                        else
                        {
                            help::illegal_param_msg(arg);
                            help::usage(cmdnm);
                            return;
                        }
                    },
                };
            }
            else if arg == "-cnt" || arg == "--count"
            {
                ix += 1;
                if argc <= ix
                {
                    help::undecided_parameter_msg(arg);
                    help::usage(cmdnm);
                    return;
                }
                /* 開始値確認 */
                arg = &args[ix];
                match arg.parse::<u32>()
                {
                    Ok(v) => setopt::count(v),
                    Err(_) => {
                        help::illegal_param_msg(arg);
                        help::usage(cmdnm);
                        return;
                    },
                };
            }
            else if arg == "-y" || arg == "--year"
            {
                ix += 1;
                if argc <= ix
                {
                    help::undecided_parameter_msg(arg);
                    help::usage(cmdnm);
                    return;
                }
                /* 開始値確認 */
                arg = &args[ix];
                match arg.parse::<u32>()
                {
                    Ok(v) => setopt::year(v),
                    Err(_) => {
                        help::illegal_param_msg(arg);
                        help::usage(cmdnm);
                        return;
                    },
                };
            }
            else if arg == "-log" || arg == "--log"
            {
                ix += 1;
                if argc <= ix
                {
                    help::undecided_parameter_msg(arg);
                    help::usage(cmdnm);
                    return;
                }
                arg = &args[ix];
                if arg.len() != 0
                {setopt::logfile(String::from(arg));}
                else
                {
                    help::usage(cmdnm);
                    return;
                }
            }
            else
            {
                help::illegal_option_msg(arg);
                help::usage(cmdnm);
                return;
            }

            ix += 1;
            if argc <= ix
            {break;}
        }
    }

    help::definite_param();

    prime::start();
}
