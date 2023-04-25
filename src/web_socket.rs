use actix::{Actor, StreamHandler, Addr, prelude::*};
use actix_web::{Error, HttpRequest, HttpResponse, web};
use std::sync::mpsc;
use std::process::Command;
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config, Event, EventKind,event::ModifyKind};
use actix_web_actors::ws;
use std::path::Path;

/// Define Websocket actor for hot reloading
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, _ctx: &mut ws::WebsocketContext<Self>) {
       println!("Actor is alive");
    }
    fn stopped(&mut self, _ctx: &mut ws::WebsocketContext<Self>) {
       println!("Actor is stopped");
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                println!("received text in websocket {msg:?}");
                ctx.pong(&msg)
            },
            Ok(ws::Message::Text(text)) => {
                println!("received text in websocket {text:?}");
                ctx.text(text)
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _e => {
                // this most likely just a message from the browser saying the connection was closed
                //println!("weird msg {:?}",e);
                 ()
            },
        }
    }
}

#[derive(Debug)]
struct Ping;

impl Message for Ping {
    type Result = Result<bool, std::io::Error>;
}


/// Define handler for `Ping` message
impl Handler<Ping> for MyWs {
    type Result = Result<bool, std::io::Error>;
    fn handle(&mut self, msg: Ping, ctx: &mut ws::WebsocketContext<Self>) -> Self::Result {
        println!("Ping for muh ChangeWatcher received, muhssage {msg:?}");
        ctx.text("reload");
        Ok(true)
    }
}


fn watch_for_changes<P: AsRef<Path>>(actor: Addr<MyWs>, folder_to_watch:P) -> notify::Result<()> {
    let (tx, rx) = mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(&folder_to_watch.as_ref(), RecursiveMode::Recursive)?;
    if let Ok(res) = rx.recv() {
        let Event { kind, paths , .. } = res?;
        if let EventKind::Modify(metadata) = kind {
            watcher.unwatch(&folder_to_watch.as_ref())?;
            println!("File changed in target folder, metadata: {metadata:?}, paths: {paths:?}");
            
            for path in paths {
                if let Some(ext) = path.extension() {
                    // run railwind depending on file
                    if ext == "html" {
                        println!("Found a html file: {:?}\n eggxecute railwind!", path);
                        eggxecute_railwind()?;
                    }
                }
            }
            // Ping for hot reload
            actor.do_send(Ping);
            // Do what you gottta do and then start watching files again 
            watcher.watch(&folder_to_watch.as_ref(), RecursiveMode::Recursive)?;
        } else {
            println!("Event is not of kind modify {kind:?}");
        }
    }
    Ok(())
}


fn eggxecute_railwind() -> std::io::Result<()>{
    let output = Command::new("railwind")
        .arg("-o")
        .arg("ui/railwind.css")
        .output()?;
    println!("Command output: {:?}", output);
    Ok(())
}

pub async fn my_web_socket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (addr,http) = ws::start_with_addr(MyWs{}, &req, stream)?;
    // make this CLI friendly later
    let path = String::from("ui");
    std::thread::spawn(||{
        watch_for_changes(addr,path).unwrap();
    });
    Ok(http)
}


