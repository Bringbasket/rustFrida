use common::{
    read_frame, write_frame, Error, Hello, Result, FRAME_KIND_CMD, FRAME_KIND_COMPLETE, FRAME_KIND_EVAL_ERR,
    FRAME_KIND_EVAL_OK, FRAME_KIND_LOG,
};
use native_api::enumerate_images;
use objc_api::ObjcApi;
use quickjs_runtime::QuickJsRuntime;

#[cfg(unix)]
use std::os::unix::net::UnixStream;
#[cfg(unix)]
use std::os::unix::io::FromRawFd;

struct AgentState {
    runtime: QuickJsRuntime,
    objc: ObjcApi,
}

impl AgentState {
    fn new() -> Self {
        Self {
            runtime: QuickJsRuntime::new(),
            objc: ObjcApi::new(),
        }
    }

    fn execute(&mut self, command: &str) -> Result<AgentReply> {
        if command == "ping" {
            return Ok(AgentReply::Log("pong".into()));
        }
        if command == "jsinit" {
            return Ok(AgentReply::EvalOk(self.runtime.initialize()?));
        }
        if let Some(script) = command.strip_prefix("loadjs ").or_else(|| command.strip_prefix("jseval ")) {
            return Ok(AgentReply::EvalOk(self.runtime.eval(script)?));
        }
        if let Some(prefix) = command.strip_prefix("jscomplete ") {
            return Ok(AgentReply::Complete(self.runtime.complete(prefix)));
        }
        if command == "objc.classes" {
            let payload = self.objc.enumerate_classes()?.join("\n");
            return Ok(AgentReply::EvalOk(payload));
        }
        if let Some(name) = command.strip_prefix("objc.classExists ") {
            return Ok(AgentReply::EvalOk(self.objc.class_exists(name).to_string()));
        }
        if let Some(name) = command.strip_prefix("objc.selector ") {
            return Ok(AgentReply::EvalOk(format!("0x{:x}", self.objc.selector(name)?)));
        }
        if command == "native.images" {
            let payload = enumerate_images()?
                .into_iter()
                .map(|image| format!("0x{:x} {}", image.slide, image.name))
                .collect::<Vec<_>>()
                .join("\n");
            return Ok(AgentReply::EvalOk(payload));
        }
        if command == "exit" {
            return Ok(AgentReply::Log("bye".into()));
        }

        Err(Error::InvalidArgument(format!("unknown agent command: {command}")))
    }
}

enum AgentReply {
    Log(String),
    Complete(Vec<String>),
    EvalOk(String),
}

#[cfg(unix)]
fn send_reply(stream: &mut UnixStream, reply: AgentReply) -> Result<()> {
    match reply {
        AgentReply::Log(line) => write_frame(stream, FRAME_KIND_LOG, line.as_bytes()),
        AgentReply::Complete(items) => write_frame(stream, FRAME_KIND_COMPLETE, items.join("\t").as_bytes()),
        AgentReply::EvalOk(result) => write_frame(stream, FRAME_KIND_EVAL_OK, result.as_bytes()),
    }
}

#[cfg(unix)]
fn run_agent_loop(fd: i32) -> Result<()> {
    let mut stream = unsafe { UnixStream::from_raw_fd(fd) };
    write_frame(&mut stream, FRAME_KIND_HELLO, &Hello::ios_default().encode())?;

    let mut state = AgentState::new();
    loop {
        let (kind, payload) = read_frame(&mut stream)?;
        if kind != FRAME_KIND_CMD {
            write_frame(&mut stream, FRAME_KIND_EVAL_ERR, b"unexpected frame kind")?;
            continue;
        }

        let command = String::from_utf8(payload)
            .map_err(|_| Error::Protocol("command payload is not valid utf-8".into()))?;
        let exit = command.trim() == "exit";
        match state.execute(command.trim()) {
            Ok(reply) => send_reply(&mut stream, reply)?,
            Err(err) => write_frame(&mut stream, FRAME_KIND_EVAL_ERR, err.to_string().as_bytes())?,
        }
        if exit {
            break;
        }
    }
    Ok(())
}

#[cfg(not(unix))]
fn run_agent_loop(_fd: i32) -> Result<()> {
    Err(Error::Unsupported(
        "agent transport currently expects a Unix domain socket fd".into(),
    ))
}

#[no_mangle]
pub extern "C" fn ios_agent_entry(fd: i32) -> i32 {
    match run_agent_loop(fd) {
        Ok(()) => 0,
        Err(_) => -1,
    }
}
