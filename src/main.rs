use swayipc::{Connection, EventType, Fallible, Workspace};

fn main() -> Fallible<()> {
    let subs = [EventType::Workspace];
    let mut cmd_con = Connection::new()?;
    let mut ws = cmd_con.get_workspaces()?;
    println!("{}", get_ws_layout(&mut ws));
    for _event in Connection::new()?.subscribe(subs)? {
        match cmd_con.get_workspaces() {
            Ok(mut ws) => println!("{}", get_ws_layout(&mut ws)),
            Err(e) => println!("Error: {e}"),
        }
    }
    Ok(())
}

fn get_ws_layout(ws: &mut Vec<Workspace>) -> String {
    ws.sort_by(|a, b| {
        if a.rect.x == b.rect.x {
            a.num.cmp(&b.num)
        } else {
            a.rect.x.cmp(&b.rect.x)
        }
    });

    let mut out = String::from(&ws[0].output);
    let mut erg = String::new();

    for w in ws {
        if w.output != out {
            out = String::from(&w.output);
            erg = format!("{erg} |");
        }
        let name = set_urgent(&set_visible(w.name.trim(), w.visible), w.urgent);

        erg = format!("{erg} {name}");
    }

    erg
}

fn set_urgent(name: &str, urgent: bool) -> String {
    if urgent {
        format!("!{name}!")
    } else {
        String::from(name)
    }
}

fn set_visible(name: &str, visible: bool) -> String {
    if visible {
        format!("[{name}]")
    } else {
        String::from(name)
    }
}
