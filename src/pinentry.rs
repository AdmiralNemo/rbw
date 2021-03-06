use crate::prelude::*;

use tokio::io::AsyncWriteExt as _;

pub async fn getpin(
    prompt: &str,
    desc: &str,
    err: Option<&str>,
    tty: Option<&str>,
) -> Result<crate::locked::Password> {
    let mut opts = tokio::process::Command::new("pinentry");
    let opts = opts
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped());
    let opts = if let Some(tty) = tty {
        opts.args(&["-T", tty])
    } else {
        opts
    };
    let mut child = opts.spawn().context(crate::error::Spawn)?;
    // unwrap is safe because we specified stdin as piped in the command opts
    // above
    let mut stdin = child.stdin.take().unwrap();

    let mut ncommands = 1;
    stdin
        .write_all(b"SETTITLE rbw\n")
        .await
        .context(crate::error::WriteStdin)?;
    ncommands += 1;
    stdin
        .write_all(format!("SETPROMPT {}\n", prompt).as_bytes())
        .await
        .context(crate::error::WriteStdin)?;
    ncommands += 1;
    stdin
        .write_all(format!("SETDESC {}\n", desc).as_bytes())
        .await
        .context(crate::error::WriteStdin)?;
    ncommands += 1;
    if let Some(err) = err {
        stdin
            .write_all(format!("SETERROR {}\n", err).as_bytes())
            .await
            .context(crate::error::WriteStdin)?;
        ncommands += 1;
    }
    stdin
        .write_all(b"GETPIN\n")
        .await
        .context(crate::error::WriteStdin)?;
    ncommands += 1;
    drop(stdin);

    let mut buf = crate::locked::Vec::new();
    buf.extend(std::iter::repeat(0));
    // unwrap is safe because we specified stdout as piped in the command opts
    // above
    let len = read_password(
        ncommands,
        buf.data_mut(),
        child.stdout.as_mut().unwrap(),
    )
    .await?;
    buf.truncate(len);

    child.await.context(crate::error::PinentryWait)?;

    Ok(crate::locked::Password::new(buf))
}

async fn read_password<
    R: tokio::io::AsyncRead + tokio::io::AsyncReadExt + Unpin,
>(
    mut ncommands: u8,
    data: &mut [u8],
    mut r: R,
) -> Result<usize> {
    let mut len = 0;
    loop {
        let nl = data.iter().take(len).position(|c| *c == b'\n');
        if let Some(nl) = nl {
            if data.starts_with(b"OK") {
                if ncommands == 1 {
                    len = 0;
                    break;
                } else {
                    data.copy_within((nl + 1).., 0);
                    len -= nl + 1;
                    ncommands -= 1;
                }
            } else if data.starts_with(b"D ") {
                data.copy_within(2..nl, 0);
                len = nl - 2;
                break;
            } else if data.starts_with(b"ERR ") {
                let line: Vec<u8> = data.iter().take(nl).copied().collect();
                let line = String::from_utf8(line).unwrap();
                let mut split = line.splitn(3, ' ');
                let _ = split.next(); // ERR
                let code = split.next();
                match code {
                    Some("83886179") => {
                        return Err(Error::PinentryCancelled);
                    }
                    Some(code) => {
                        if let Some(error) = split.next() {
                            return Err(Error::PinentryErrorMessage {
                                error: error.to_string(),
                            });
                        } else {
                            return Err(Error::PinentryErrorMessage {
                                error: format!("unknown error ({})", code),
                            });
                        }
                    }
                    None => {
                        return Err(Error::PinentryErrorMessage {
                            error: "unknown error".to_string(),
                        });
                    }
                }
            } else {
                return Err(Error::FailedToParsePinentry {
                    out: String::from_utf8_lossy(data).to_string(),
                });
            }
        } else {
            let bytes = r
                .read(&mut data[len..])
                .await
                .context(crate::error::PinentryReadOutput)?;
            len += bytes;
        }
    }

    Ok(len)
}

#[test]
fn test_read_password() {
    let good_inputs = &[
        (0, &b"D super secret password\n"[..]),
        (4, &b"OK\nOK\nOK\nD super secret password\nOK\n"[..]),
        (12, &b"OK\nOK\nOK\nOK\nOK\nOK\nOK\nOK\nOK\nOK\nOK\nD super secret password\nOK\n"[..]),
        (24, &b"OK\nOK\nOK\nOK\nOK\nOK\nOK\nOK\nOK\nOK\nOK\nOK\nOK\nOK\nOK\nOK\nOK\nOK\nOK\nOK\nOK\nOK\nOK\nD super secret password\nOK\n"[..]),
    ];
    for (ncommands, input) in good_inputs {
        let mut buf = [0; 64];
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let len = read_password(*ncommands, &mut buf, &input[..])
                .await
                .unwrap();
            assert_eq!(&buf[0..len], b"super secret password");
        });
    }

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let mut buf = [0; 64];
        let len = read_password(4, &mut buf, &b"OK\nOK\nOK\nOK\n"[..])
            .await
            .unwrap();
        assert_eq!(len, 0);
    })
}
