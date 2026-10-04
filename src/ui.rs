use crate::{
    game::{Game, INITIAL},
    scene::Avatar,
};

pub const NEED_NAMES: [&str; 5] = ["Energía", "Hambre", "Higiene", "Diversión", "Vejiga"];
pub const NEED_COLORS: [&str; 5] = ["#76b7b2", "#e9ad62", "#72a9d8", "#d080a7", "#9aafdd"];
pub const NEED_ICONS: [&str; 5] = ["⚡", "●", "◈", "★", "◉"];
pub fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}
pub fn needs(values: &[f64; 5], count: usize) -> String {
    (0..count).map(|i| format!(r#"<div class="need"><span>{}</span><div><label>{}</label><i><em id="need-{i}" style="width:{}%;background:{}"></em></i></div></div>"#,NEED_ICONS[i],NEED_NAMES[i],values[i],NEED_COLORS[i])).collect()
}
pub fn creator(avatar: &Avatar) -> String {
    let options = ["delgado", "adulto", "atlético"]
        .iter()
        .map(|body| {
            format!(
                "<option{}>{body}</option>",
                if *body == avatar.body {
                    " selected"
                } else {
                    ""
                }
            )
        })
        .collect::<String>();
    let colors = [
        ("Piel", "skin", &avatar.skin),
        ("Cabello", "hair", &avatar.hair),
        ("Camisa", "shirt", &avatar.shirt),
        ("Pantalón", "pants", &avatar.pants),
    ]
    .iter()
    .map(|(label, id, color)| {
        format!(
            r#"<label>{label}<input id="{id}" type="color" value="{}"></label>"#,
            escape(color)
        )
    })
    .collect::<String>();
    format!(
        r#"<section><div class="creator-head"><div><small>CREAR PERSONAJE</small><h1>Tu nueva vida</h1></div></div><label>Nombre<input id="name" value="{}"></label><label>Complexión<select id="body">{options}</select></label><div class="swatches">{colors}</div><button class="play" id="play">ENTRAR A LA CASA</button></section>"#,
        escape(&avatar.name)
    )
}
pub fn shell(avatar: &Avatar) -> String {
    let name = escape(&avatar.name);
    let upper = escape(&avatar.name.to_uppercase());
    let letter = escape(
        &avatar
            .name
            .chars()
            .next()
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_default(),
    );
    format!(
        r#"<main class="game-shell"><div id="scene" class="scene" aria-label="Habitación 3D interactiva"></div><header class="topbar"><div class="brand"><span class="house">◆</span><div><b id="house-name">CASA DE {upper}</b><small>Martes · Primavera</small></div></div><div class="time"><button id="pause">Ⅱ</button><b id="clock">09:42</b><div class="speed">▶ ▶ ▶</div></div><button class="menu" id="menu">👤</button></header><aside class="needs"><div class="portrait"><span>◆</span><div class="avatar" id="avatar-letter">{letter}</div><div><b id="avatar-name">{name}</b><small id="action">Nada pendiente</small></div></div><div id="needs-list">{}</div></aside><div class="hint">Autonomía activa · Pulsa un mueble para darle una indicación</div><nav class="actions"><button id="eat"><span>◌</span>Comer</button><button id="tv"><span>▣</span>Televisión</button><button id="bed"><span>▰</span>Dormir</button><button id="shower"><span>♨</span>Ducha</button><button id="toilet"><span>◉</span>Baño</button></nav><div class="creator" id="creator">{}</div></main>"#,
        needs(&INITIAL, 4),
        creator(avatar)
    )
}
pub fn status(game: &Game) -> (&str, &str) {
    (&game.action, &game.displayed_clock)
}
