use std::ffi::OsStr;
use once_cell::sync::OnceCell;

static OUTOFFORM: OnceCell<String> = OnceCell::new();
static DEF_OUTOFFORM: OnceCell<String> = OnceCell::new();
static PRIMESTRT: OnceCell<u32> = OnceCell::new();
static DEF_PRIMESTRT: OnceCell<u32> = OnceCell::new();
static PRIMEEND: OnceCell<u32> = OnceCell::new();
static DEF_PRIMEEND: OnceCell<u32> = OnceCell::new();
static COUNT: OnceCell<u32> = OnceCell::new();
static DEF_COUNT: OnceCell<u32> = OnceCell::new();
static YEAR: OnceCell<u32> = OnceCell::new();
static DEF_YEAR: OnceCell<u32> = OnceCell::new();
static LOGFILE: OnceCell<String> = OnceCell::new();
static DEF_LOGFILE: OnceCell<String> = OnceCell::new();

pub fn usage(cmdnm:&OsStr)
{
    println!("素数カレンダー: 0001/01/01からの日数をカウントし、それが素数となる時の年月日を表示する");
    println!("{:?}:", cmdnm);
    println!("\t[-of | --outformat] [nrml | bnnr | none] : 表示方法選択 Def.'nrml'");
    println!("\t[-prm | --prime] <開始値> [<終了値>] : 素数範囲指定 (終了値 Def. 無限)");
    println!("\t[-cnt | --count] <回数> : 表示回数指定 Def. 無限");
    println!("\t[-y | --year] <西暦> : 表示対象西暦 Def. 0年");
    println!("\t[-log | --log] <ファイル名> : ログファイル出力 Def. ログ出力無し");

    //definite_param();
}

pub fn illegal_param_msg(arg:&str)
{
    println!("Illegal parameter : {}", arg);
}
pub fn undecided_parameter_msg(arg:&str)
{
    println!("Option {} parameter undecided", arg);
}
pub fn illegal_option_msg(arg:&str)
{
    println!("Illegal option : {}", arg);
}

pub fn definite_param()
{
    println!("outFormat:{} ", getopt::outformat());
    println!("primeTermStart:{} ", getopt::primestart());
    println!("primeTermEnd:{}", getopt::primeend());
    println!("count:{}", getopt::count());
    println!("year:{}", getopt::year());
    println!("log:{}", getopt::logfile());
}

pub mod setopt
{
    pub fn init()
    {
        crate::help::DEF_OUTOFFORM.set("nrml".to_string()).unwrap();
        crate::help::DEF_PRIMESTRT.set(1).unwrap();
        crate::help::DEF_PRIMEEND.set(0).unwrap();
        crate::help::DEF_COUNT.set(0).unwrap();
        crate::help::DEF_YEAR.set(0).unwrap();
        crate::help::DEF_LOGFILE.set("".to_string()).unwrap();
    }

    pub fn outformat(arg:String)
    {
        crate::help::OUTOFFORM.set(arg).unwrap();
    }
    pub fn primestart(arg:u32)
    {
        crate::help::PRIMESTRT.set(arg).unwrap();
    }
    pub fn primeend(arg:u32)
    {
        crate::help::PRIMEEND.set(arg).unwrap();
    }
    pub fn count(arg:u32)
    {
        crate::help::COUNT.set(arg).unwrap();
    }
    pub fn year(arg:u32)
    {
        crate::help::YEAR.set(arg).unwrap();
    }
    pub fn logfile(arg:String)
    {
        crate::help::LOGFILE.set(arg).unwrap();
    }
}

pub mod getopt
{
    pub fn outformat() -> &'static String
    {
        let vlu = match crate::help::OUTOFFORM.get()
        {
            Some(v) => v,
            None => crate::help::DEF_OUTOFFORM.get().unwrap(),
        };
        vlu
    }
    pub fn primestart() -> u32
    {
        let vlu = match crate::help::PRIMESTRT.get()
        {
            Some(v) => *v,
            None => *crate::help::DEF_PRIMESTRT.get().unwrap(),
        };
        vlu
    }
    pub fn primeend() -> u32
    {
        let vlu = match crate::help::PRIMEEND.get()
        {
            Some(v) => *v,
            None => *crate::help::DEF_PRIMEEND.get().unwrap(),
        };
        vlu
    }
    pub fn count() -> u32
    {
        let vlu = match crate::help::COUNT.get()
        {
            Some(v) => *v,
            None => *crate::help::DEF_COUNT.get().unwrap(),
        };
        vlu
    }
    pub fn year() -> u32
    {
        let vlu = match crate::help::YEAR.get()
        {
            Some(v) => *v,
            None => *crate::help::DEF_YEAR.get().unwrap(),
        };
        vlu
    }
    pub fn logfile() -> &'static String
    {
        let vlu = match crate::help::LOGFILE.get()
        {
            Some(v) => v,
            None => crate::help::DEF_LOGFILE.get().unwrap(),
        };
        vlu
    }
}
