
use koyomi;
use std::iter::Iterator;
use std::process;

use crate::help::getopt;
use crate::output;

pub fn start()
{
    println!("Start calculation");

    let primecnt = getopt::count();
    let primestrt = getopt::primestart();
    let primeend = getopt::primeend();


    let mut clcvl = primestrt;
    let mut cnt :u32 = 0;

    if primeend == 0
    {
        if primecnt == 0
        {
            println!("forever calculat from {}", primestrt);
            loop
            {
                (cnt, clcvl) = do_prime(cnt, clcvl);
            }
        }
        else
        {
            println!("{} times calculat from {}", primecnt, primestrt);
            loop
            {
                (cnt, clcvl) = do_prime(cnt, clcvl);
                if primecnt <= cnt
                {
                    let outopt = getopt::outformat();
                    if outopt == "bnnr"
                    {
                        let mtdays = output::get::premtdays();
                        banner_fillout(mtdays);
                    }
                    break;
                }
            }
        }
    }
    else
    {
        if primecnt == 0
        {
            println!("forever calculat from {}", primestrt);
            loop
            {
                (cnt, clcvl) = do_prime(cnt, clcvl);
                if primeend < clcvl
                {
                    let outopt = getopt::outformat();
                    if outopt == "bnnr"
                    {
                        let mtdays = output::get::premtdays();
                        banner_fillout(mtdays);
                    }
                    break;
                }
            }
        }
        else
        {
            println!("{} times calculat from {}", primecnt, primestrt);
            loop
            {
                (cnt, clcvl) = do_prime(cnt, clcvl);
                if primecnt <= cnt || primeend < clcvl
                {
                    let outopt = getopt::outformat();
                    if outopt == "bnnr"
                    {
                        let mtdays = output::get::premtdays();
                        banner_fillout(mtdays);
                    }
                    break;
                }
            }
        }
    }
}

pub fn do_prime(mut cnt:u32, mut clcvl:u32) -> (u32, u32)
{
    let chkmax = (clcvl as f64).sqrt() + 1.0;
    for dv in 2..(chkmax as u32)
    {
        if (clcvl % dv) == 0
        {
            clcvl += 1;
            return (cnt, clcvl);
        }
    }
    //println!("{} prime!", clcvl);

    day2dal(clcvl);

    let prmyr = getopt::year();
    if prmyr == 0
    {cnt += 1;}
    else if prmyr != 0 && prmyr <= output::get::preyr().try_into().unwrap()
    {cnt += 1;}

    clcvl += 1;

    return (cnt, clcvl);
}

const MONDYS : [u32;12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const LPMONDYS : [u32;12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
/***
fn print_type<T>(_:T)
{
    println!("{}", std::any::type_name::<T>());
}
***/
fn banner_fillout(mtdays:u32)
{
    let year = output::get::preyr();
    let mnth = output::get::premt();
    for dd in output::get::mtday()..32
    {
        if dd <= mtdays
        {print!("{:02}", dd);}
        else
        {print!("@@");}
    }
    print!("\n{:04}/{:02}/{:02}\n", year, mnth, output::get::mtday());
}
fn day2dal(clcvl:u32)
{
    let logf = getopt::logfile();
    let outyear = getopt::year();
    let mut dday = clcvl;
    let mut year:i32 = 1;
    let mut mt = 0;

    //print_type(&mt);

    loop
    {
        if koyomi::is_leap(year)
        {
            if 366 < dday
            {dday -= 366;}
            else
            {
                for (i, m) in LPMONDYS.iter().enumerate()
                {
                    //print!("{:02} ", i);
                    if m < &dday
                    {dday -= m}
                    else
                    {
                        mt = i + 1;
                        break;
                    }
                }
                //print!("\n");
                break;
            }
        }
        else
        {
            if 365 < dday
            {dday -= 365;}
            else
            {
                for (i, m) in MONDYS.iter().enumerate()
                {
                    if m < &dday
                    {dday -= m}
                    else
                    {
                        mt = i + 1;
                        break;
                    }
                }
                break;
            }
        }
        year += 1;
    }
    //print_type(&mt);

    let outopt = getopt::outformat();
    if (outyear == 0) || (outyear == (year as u32))
    {
        if outopt == "bnnr"
        {
            let mnthx: usize = (mt-1).try_into().unwrap();
            let mtdays:u32;
            if koyomi::is_leap(year)
            {mtdays = LPMONDYS[mnthx];}
            else
            {mtdays = MONDYS[mnthx];}

            output::bannerout(clcvl, year, mt.try_into().unwrap(), dday, mtdays, logf.to_string());
        }
        else if outopt == "nrml"
        {output::normalout(clcvl, year, mt.try_into().unwrap(), dday, logf.to_string());}
    }
    else if (outyear != 0) && (outyear < (year as u32))
    {
        if outopt == "bnnr"
        {
            let mnthx: usize = (mt-1).try_into().unwrap();
            let mtdays:u32;
            if koyomi::is_leap(year)
            {mtdays = LPMONDYS[mnthx];}
            else
            {mtdays = MONDYS[mnthx];}

            output::bannerout(clcvl, year, mt.try_into().unwrap(), dday, mtdays, logf.to_string());

            banner_fillout(mtdays);
        }
        else if outopt == "nrml"
        {output::normalout(clcvl, year, mt.try_into().unwrap(), dday, logf.to_string());}
        process::exit(0x0100);
    }
    else
    {output::progresout(clcvl, year);}
}
/***
pub fn term_prime(strv:u32, endv:u32)
{
    if (outopt != 'none') and (0 < len(lgfpre))
    {
        fnm = lgfpre+'_'+str(strv)+'_'+str(endv)+'.log'
        print('log:{}'.format(fnm))
        logf = open(fnm, 'w')
    }
    else
    {
        logf = sys.stdout
    }
    cnt = 0
    vl = strv
    while vl <= endv
    {
        cnt, vl = do_prime(cnt, vl, logf)
        if (0 < outmax) and (outmax <= cnt):
            break
    }
}
***/
