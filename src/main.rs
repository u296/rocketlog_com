use std::{time::{Duration, Instant}, io::{Write, Read}};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    join,
};

fn unpack_floats(buf: [u8; 13]) -> [f32; 3] {
    let mut xbuf = [0; 4];
    let mut ybuf = [0; 4];
    let mut zbuf = [0; 4];

    xbuf.copy_from_slice(&buf[0..4]);
    ybuf.copy_from_slice(&buf[4..8]);
    zbuf.copy_from_slice(&buf[8..12]);

    [
        f32::from_le_bytes(xbuf),
        f32::from_le_bytes(ybuf),
        f32::from_le_bytes(zbuf),
    ]
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    // disable echo for the board
    std::process::Command::new("stty")
        .arg("--file=/dev/ttyACM0")
        .arg("-echo")
        .arg("brkint")
        .arg("imaxbel")
        .spawn()?
        .wait()?;



    let mut device = std::fs::File::options().read(true).write(true).create(false).open("/dev/ttyACM0")?;
    let mut results = std::fs::File::create("results.csv")?;

    let mut buf2 = [0; 13];

    let start = std::time::Instant::now();

    writeln!(results, "time,x,y,z")?;

    loop {
        device.write_all(b"poll")?;
        device.flush()?;
        //return Ok(());
        let count = device.read(&mut buf2)?;

        if count < 12 {
            println!("MISSING BYTES");
        }

        let mut acc = unpack_floats(buf2);

        for i in 0..3 {
            if acc[i].abs() > 100.0 {
                acc[i] = 0.0;
            }
        }

        println!("{}, {}, {}", acc[0], acc[1], acc[2]);

        writeln!(results, "\"{}\",\"{}\",\"{}\",\"{}\"", (Instant::now() - start).as_millis(), acc[0], acc[1], acc[2])?;

        std::thread::sleep(Duration::from_millis(5));
    }

    Ok(())
}
