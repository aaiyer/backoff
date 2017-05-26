extern crate backoff;

use backoff::ExponentialBackOff;
use backoff::retry::Operation;
use backoff::error::Error;

use std::io;

#[test]
fn retry() {
    let mut i = 0;
    let success_on = 3;

    {
        let mut f = || -> Result<(), Error<io::Error>> {
            i+=1;
            if i == success_on {
                return Ok(());
            }
    
            return Err(Error::Transient(io::Error::new(io::ErrorKind::Other, "err")));
        };
    
        let backoff = ExponentialBackOff::default();
        let _ = f.retry(backoff).ok().unwrap();
    }

    assert_eq!(i, success_on);
}

#[test]
fn permanent_error_immediately_returned() {
    let mut f = || -> Result<(), Error<io::Error>> {
        Err(Error::Permanent(io::Error::new(io::ErrorKind::Other, "err")))
    };

    let backoff = ExponentialBackOff::default();
    match f.retry(backoff).err().unwrap(){
        Error::Permanent(_) => (),
        other => panic!("{}", other),
    }
}