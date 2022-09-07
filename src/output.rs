
use once_cell::sync::OnceCell;

static ESC    : OnceCell<String> = OnceCell::new();
static CHRRED : OnceCell<String> = OnceCell::new();
static BCKBLK : OnceCell<String> = OnceCell::new();
static ENDSGR : OnceCell<String> = OnceCell::new();

static mut PREYR: i32 = 0;
static mut PREMT: i32 = 0;
static mut MTDAY: u32 = 0;
static mut PREMTDAYS: u32 = 0;


pub fn init()
{
    crate::output::ESC.set("\x1b[".to_string()).unwrap();
    crate::output::CHRRED.set("\x1b[31m".to_string()).unwrap();
    crate::output::BCKBLK.set("\x1b[40m".to_string()).unwrap();
    crate::output::ENDSGR.set("\x1b[0m".to_string()).unwrap();

    unsafe {crate::output::PREYR = 0;}
    unsafe {crate::output::PREMT = 0;}
    unsafe {crate::output::MTDAY = 0;}
    unsafe {crate::output::PREMTDAYS = 0;}
}

mod set
{
    pub fn preyr(yr:i32)
    {unsafe {crate::output::PREYR = yr;}}
    pub fn premt(mt:i32)
    {unsafe {crate::output::PREMT = mt;}}
    pub fn mtday(dy:u32)
    {unsafe {crate::output::MTDAY = dy;}}
    pub fn premtdays(dys:u32)
    {unsafe {crate::output::PREMTDAYS = dys;}}
}
pub mod get
{
    pub fn esc() -> &'static String
    {crate::output::ESC.get().unwrap()}
    pub fn chrred() -> &'static String
    {crate::output::CHRRED.get().unwrap()}
    pub fn bckblk() -> &'static String
    {crate::output::BCKBLK.get().unwrap()}
    pub fn endsgr() -> &'static String
    {crate::output::ENDSGR.get().unwrap()}

    pub fn preyr() -> i32
    {unsafe {crate::output::PREYR}}
    pub fn premt() -> i32
    {unsafe {crate::output::PREMT}}
    pub fn mtday() -> u32
    {unsafe {crate::output::MTDAY}}
    pub fn premtdays() -> u32
    {unsafe {crate::output::PREMTDAYS}}
}
//const PRMPT : [char;4] = ['-', '\\', '|', '/'];
pub fn progresout(prm:u32, yr:i32)
{
    /***
    let stx : usize = (prm % PRMPT.len() as u32).try_into().unwrap();
    print!("{}\r", PRMPT[stx]);
    ***/
    print!("{}: {:04}\r", prm, yr);
}

pub fn normalout(prm:u32, yr:i32, mt:i32, dday:u32, logf:String)
{
    //print('{}: {:04}/{:02}/{:02}'.format(prm, yr, mt, dday), file=logf)
    println!("{}: {:04}/{:02}/{:02}", prm, yr, mt, dday);
}

pub fn  bannerout(prm:u32, yr:i32, mt:i32, dday:u32, mtdays:u32, logf:String)
{
    //println!("mt {} dday {} mtdays {}", mt, dday, mtdays);

    if (get::preyr() != yr) || (get::premt() != mt)
    {
        if 0 < get::mtday()
        {
            print!("{}", get::bckblk());
            let premtdays = get::premtdays();
            let mut dd = get::mtday();
            while dd <= 31
            {
                if dd <= premtdays
                {print!("{:02}", dd);}
                else
                {print!("@@");}
                dd += 1;
                set::mtday(dd);
            }
            print!("{}", get::endsgr());
        }
        set::mtday(1);

        print!("\r\n{}: ", prm);
        print!("{:04}/{:02}[{:02}]/", yr,mt,mtdays);

        print!("{}", get::bckblk());

        let mut dd = get::mtday();
        while dd < dday.try_into().unwrap()
        {
            //if dd <= mtdays
            {print!("{:02}", dd);}
            /***
            else
            {print!("@@");}
            ***/
            dd += 1;
            set::mtday(dd);
        }
        print!("{}", get::endsgr());

        print!("{}{:02}{}", get::chrred(), dday, get::endsgr());
        let val = dday + 1;
        set::mtday(val);
        set::preyr(yr);
        set::premt(mt);
        set::premtdays(mtdays);
    }
    else
    {
        print!("{}", get::bckblk());
        let mut dd = get::mtday();
        while dd < dday.try_into().unwrap()
        {
            //if dd <= mtdays
            {print!("{:02}", dd);}
            dd += 1;
            set::mtday(dd);
        }
        print!("{}", get::endsgr());

        print!("{}{:02}{}", get::chrred(), dday, get::endsgr());
        let val = dday + 1;
        set::mtday(val);
    }
}
