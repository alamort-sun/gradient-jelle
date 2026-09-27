//! Local chat through an explicitly selected, already installed Ollama model.
use gradient_jelle::{
    dialogue::{DialogueError, Session, TextGenerator},
    JelleState,
};
use std::{
    io::{self, BufRead, Write},
    path::Path,
    process::{Command, Stdio},
};
struct Ollama {
    model: String,
}
impl TextGenerator for Ollama {
    fn generate(&mut self, prompt: &str) -> Result<String, DialogueError> {
        let mut child = Command::new("ollama")
            .args(["run", &self.model, "--nowordwrap", "--hidethinking"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| DialogueError::Backend(e.to_string()))?;
        let write = child.stdin.take().unwrap().write_all(prompt.as_bytes());
        if let Err(e) = write {
            let _ = child.kill();
            let _ = child.wait();
            return Err(DialogueError::Backend(e.to_string()));
        }
        let out = child
            .wait_with_output()
            .map_err(|e| DialogueError::Backend(e.to_string()))?;
        if !out.status.success() {
            return Err(DialogueError::Backend(format!(
                "Ollama exited {}",
                out.status
            )));
        }
        String::from_utf8(out.stdout).map_err(|e| DialogueError::Backend(e.to_string()))
    }
}
fn save(path: &Path, session: &Session) -> Result<(), Box<dyn std::error::Error>> {
    // An exclusive sidecar lock in main prevents competing sessions overwriting one another.
    let tmp = path.with_extension("json.tmp");
    std::fs::write(&tmp, session.to_json()?)?;
    std::fs::rename(tmp, path)?;
    Ok(())
}
struct Lock(std::path::PathBuf);
impl Drop for Lock {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err(
            "Usage: discordia_chat MODEL SESSION.json (Ollama must already be running)".into(),
        );
    }
    // Check installation first: ollama run otherwise auto-downloads missing models.
    let listing = Command::new("ollama").arg("list").output()?;
    if !listing.status.success() {
        return Err("Start Ollama first; no model was downloaded.".into());
    }
    if !String::from_utf8_lossy(&listing.stdout)
        .lines()
        .skip(1)
        .any(|l| l.split_whitespace().next() == Some(&args[1]))
    {
        return Err(
            "Use an exact installed model name from ollama list; downloads are not automatic here."
                .into(),
        );
    }
    let path = Path::new(&args[2]);
    let lock_path = path.with_extension("json.lock");
    let _file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&lock_path)?;
    let _lock = Lock(lock_path);
    let mut session = if path.exists() {
        Session::from_json(&std::fs::read_to_string(path)?)?
    } else {
        Session::default()
    };
    let state = JelleState(vecGradient::Vector15D::default());
    let mut backend = Ollama {
        model: args[1].clone(),
    };
    eprintln!("Discordia — /design on|off, /bold 0..1, /colour 0..1, /play 0..1, /remember NOTE, /prefer WHY, /forget, /quit. Replies are unverified model output.");
    for line in io::stdin().lock().lines() {
        let line = line?;
        if line == "/quit" {
            break;
        }
        let result: Result<(), Box<dyn std::error::Error>> = (|| {
            if let Some(v) = line.strip_prefix("/design ") {
                let mut d = session.creative_direction();
                d.design_mode = match v {
                    "on" => true,
                    "off" => false,
                    _ => return Err("Use /design on or /design off".into()),
                };
                session.set_creative_direction(d)?;
            } else if let Some(v) = line.strip_prefix("/bold ") {
                let mut d = session.creative_direction();
                d.boldness = v.parse()?;
                session.set_creative_direction(d)?;
            } else if let Some(v) = line.strip_prefix("/colour ") {
                let mut d = session.creative_direction();
                d.colour_freedom = v.parse()?;
                session.set_creative_direction(d)?;
            } else if let Some(v) = line.strip_prefix("/play ") {
                session.set_playfulness(v.parse()?)?;
            } else if let Some(note) = line.strip_prefix("/remember ") {
                session.remember(note.into())?;
            } else if let Some(reason) = line.strip_prefix("/prefer ") {
                session.prefer_last(reason.into())?;
            } else if line == "/forget" {
                session.forget();
            } else if line.starts_with('/') {
                return Err("Unknown command".into());
            } else {
                println!("{}", session.reply(&state, &line, &mut backend)?.text);
            }
            save(path, &session)
        })();
        if let Err(e) = result {
            eprintln!("{e}");
        }
    }
    Ok(())
}
