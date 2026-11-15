use std::fmt;
use std::io;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};

/// A thin error wrapper around I/O and process‐exit errors.
#[derive(Debug)]
pub enum SocatError {
    Io(io::Error),
    ExitCode(i32),
}

impl From<io::Error> for SocatError {
    fn from(e: io::Error) -> Self {
        SocatError::Io(e)
    }
}

impl fmt::Display for SocatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SocatError::Io(e) => write!(f, "I/O error: {e}"),
            SocatError::ExitCode(c) => write!(f, "socat exited with code {c}"),
        }
    }
}

impl std::error::Error for SocatError {}

#[derive(Debug)]
pub struct SocatStream {
    child: Child,
    stdin: ChildStdin,
    stdout: ChildStdout,
}

impl SocatStream {
    /// Spawn `wsl socat STDIO UNIX-CONNECT:<uds_path>` and return a Read+Write handle.
    pub fn connect(uds_path: &str) -> Result<Self, SocatError> {
        let mut child = Command::new("wsl")
            .arg("socat")
            .args(["-b", "200000"]) // 200 KiB buffer size
            .arg("STDIO")
            .arg(format!(
                "UNIX-CONNECT:{uds_path},sndbuf=65536,rcvbuf=131072"
            ))
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(SocatError::Io)?;
        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| io::Error::other("failed to open stdin"))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| io::Error::other("failed to open stdout"))?;

        Ok(SocatStream {
            child,
            stdin,
            stdout,
        })
    }

    /// Check whether socat has already exited with an error.
    pub fn check_alive(&mut self) -> Result<(), SocatError> {
        if let Some(status) = self.child.try_wait().map_err(SocatError::Io)? {
            if !status.success() {
                return Err(SocatError::ExitCode(status.code().unwrap_or(-1)));
            }
        }
        Ok(())
    }

    /// Kill the socat process (best‐effort).
    pub async fn shutdown(&mut self) -> Result<(), SocatError> {
        self.child.kill().await.map_err(SocatError::Io)?;
        Ok(())
    }

    /// Read from the stream into the provided buffer.
    pub async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stdout.read(buf).await
    }

    /// Write the provided buffer to the stream.
    pub async fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stdin.write(buf).await
    }

    /// Flush the stream.
    pub async fn flush(&mut self) -> io::Result<()> {
        self.stdin.flush().await
    }

    /// Split the stream into separate read and write halves.
    ///
    /// This allows for the read and write halves to be used independently,
    /// including being moved to different tasks.
    pub fn split(self) -> (ReadHalf<SocatStream>, WriteHalf<SocatStream>) {
        tokio::io::split(self)
    }
}

// Implement AsyncRead and AsyncWrite for SocatStream
impl AsyncRead for SocatStream {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        std::pin::Pin::new(&mut self.stdout).poll_read(cx, buf)
    }
}

impl AsyncWrite for SocatStream {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<io::Result<usize>> {
        std::pin::Pin::new(&mut self.stdin).poll_write(cx, buf)
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        std::pin::Pin::new(&mut self.stdin).poll_flush(cx)
    }

    fn poll_shutdown(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        std::pin::Pin::new(&mut self.stdin).poll_shutdown(cx)
    }
}
