//! Physical residents and reciprocal, time-based conversations backed by MSPA.
use crate::{
    game::{Frame, Game, Navigation, Phase, Vec3},
    psychology::{Psychology, Reaction, Relationship, SocialKind},
    scene::{self, Avatar},
};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Resident {
    pub id: u64,
    pub avatar: Avatar,
    pub frame: Frame,
    pub reaction: Reaction,
    pub psychology: Psychology,
    pub relationship: Relationship,
    pub action: String,
    path: VecDeque<Vec3>,
    wander_in: f64,
    wait_for: f64,
    cooldown: f64,
    needs: [f64;5],
}
#[derive(Clone, Debug)]
struct Invitation { target: u64, kind: SocialKind, expires: f64, order: u64 }
#[derive(Clone, Debug)]
struct Conversation { a: u64, b: u64, kind: SocialKind, elapsed: f64, turn: u8 }

#[derive(Debug)]
pub struct Society {
    pub residents: Vec<Resident>,
    pub selected: Option<u64>,
    pub status: String,
    pending: Option<Invitation>,
    conversation: Option<Conversation>,
    time: f64,
    decision_in: f64,
    wander_sequence: usize,
}

fn distance(a: Vec3, b: Vec3) -> f64 { (a.x-b.x).hypot(a.z-b.z) }
fn yaw(from: Vec3, target: Vec3) -> f64 { (target.x-from.x).atan2(target.z-from.z) }

impl Society {
    pub fn new(navigation: &Navigation) -> Self {
        let residents = scene::residents().into_iter().map(|(id,avatar,position)| {
            let point = Vec3::new(position[0],0.0,position[2]);
            let point = navigation.nearest(point).map(Navigation::point).unwrap_or(point);
            Resident {
                id, avatar, frame: Frame { position: point, ..Frame::default() },
                reaction: Reaction { remaining: 0.0, ..Reaction::default() },
                psychology: Psychology::new_agent(id, 0x4d454e530000+id).expect("MSPA resident"),
                relationship: Relationship::default(), action: "En casa".into(),
                path: VecDeque::new(), wander_in: 2.0+id as f64,
                wait_for: 0.0, cooldown: 0.0, needs: [85.0,82.0,90.0,75.0,90.0],
            }
        }).collect();
        Self { residents, selected: Some(2), status: "Pulsa un personaje para interactuar".into(),
            pending: None, conversation: None, time: 0.0, decision_in: 9.0, wander_sequence: 0 }
    }

    fn index(&self, id: u64) -> Option<usize> { self.residents.iter().position(|r| r.id == id) }
    fn position(&self, id: u64, game: &Game) -> Vec3 {
        if id == 1 { game.ground_position }
        else { self.residents[self.index(id).expect("resident identity")].frame.position }
    }
    fn speak(&mut self, id: u64, game: &mut Game, words: &str) {
        if id == 1 { game.say(words); }
        else if let Some(i) = self.index(id) {
            let resident = &mut self.residents[i];
            resident.reaction = Reaction { expression: resident.psychology.emotion(), words: words.into(), remaining: 6.0 };
        }
    }
    fn name<'a>(&'a self, id: u64, protagonist: &'a str) -> &'a str {
        if id == 1 { protagonist } else { &self.residents[self.index(id).unwrap()].avatar.name }
    }

    pub fn request(&mut self, target: u64, kind: SocialKind, game: &mut Game) {
        if game.paused || game.creator { return; }
        let Some(index) = self.index(target) else { return; };
        self.cancel_conversation(game);
        self.selected = Some(target);
        let destination = self.residents[index].frame.position;
        // Find an actual free meeting point, never a location inside the other person.
        let mut candidates: Vec<_> = game.navigation.free.iter().enumerate()
            .filter(|(_,free)| **free)
            .map(|(i,_)| Navigation::point(i))
            .filter(|point| (0.85..=1.35).contains(&distance(*point,destination)))
            .collect();
        candidates.sort_by(|a,b| distance(*a,game.ground_position).total_cmp(&distance(*b,game.ground_position)));
        let Some(point) = candidates.into_iter()
            .find(|point| game.navigation.route(game.ground_position,*point).is_some()) else {
                game.say("No encuentro un lugar para acercarme."); return;
            };
        self.residents[index].path.clear();
        self.residents[index].wander_in = 35.0;
        game.walk_to(point);
        self.pending = Some(Invitation { target,kind,expires: self.time+35.0,order:game.order_serial });
        self.status = format!("Acercándose a {}",self.residents[index].avatar.name);
        game.say(&format!("¡{}! Voy para allá.",self.residents[index].avatar.name));
    }

    fn start(&mut self, a: u64, b: u64, kind: SocialKind, game: &mut Game) {
        if a == 1 || b == 1 {
            game.talking = true;
            game.action = format!("Conversando con {}",self.name(if a == 1 { b } else { a },""));
        }
        for id in [a,b] {
            if let Some(i) = self.index(id) {
                self.residents[i].path.clear();
                self.residents[i].action = "Conversando".into();
            }
        }
        self.pending = None;
        self.conversation = Some(Conversation { a,b,kind,elapsed:0.0,turn:0 });
        self.status = "Conversación en curso".into();
        let opening = match kind {
            SocialKind::Greet => "¡Hola! Qué gusto verte por aquí.",
            SocialKind::Chat => "¿Cómo te está tratando el día?",
            SocialKind::Joke => "Vine a hacer algo... y ya se me olvidó qué era.",
        };
        self.speak(a,game,opening);
    }

    fn report(&mut self, conversation: &Conversation, game: &mut Game, success: bool) {
        for (id,target) in [(conversation.a,conversation.b),(conversation.b,conversation.a)] {
            if id == 1 { game.psychology.social_report(target,conversation.kind,success); }
            else if let Some(i) = self.index(id) {
                let resident = &mut self.residents[i];
                resident.psychology.social_report(target,conversation.kind,success);
                resident.cooldown = 18.0;
                resident.wander_in = 4.0;
                resident.action = "En casa".into();
                if success { resident.needs[3] = (resident.needs[3]+8.0).min(100.0); }
                resident.relationship = resident.psychology.relationship(1);
            }
        }
        if conversation.a == 1 || conversation.b == 1 {
            game.talking = false;
            if success {
                game.values[3] = (game.values[3]+8.0).min(100.0);
                game.action = "Disfrutando de la compañía".into();
                game.say("¡Me ha venido bien hablar contigo!");
            }
            game.decision_in = 5.0;
        }
    }
    fn cancel_conversation(&mut self, game: &mut Game) {
        if let Some(conversation) = self.conversation.take() {
            self.report(&conversation,game,false);
        }
        self.pending = None;
    }

    pub fn tick(&mut self, game: &mut Game, dt: f64) {
        if game.creator || game.paused { return; }
        let dt = if dt.is_finite() { dt.clamp(0.0,0.05) } else { 0.0 };
        self.time += dt;
        for resident in &mut self.residents {
            for need in &mut resident.needs { *need = (*need-dt*0.015).max(0.0); }
            resident.psychology.advance(dt,resident.needs);
            resident.reaction.remaining = (resident.reaction.remaining-dt).max(0.0);
            resident.reaction.expression = resident.psychology.emotion();
            resident.cooldown = (resident.cooldown-dt).max(0.0);
        }
        if let Some(invitation) = self.pending.clone() {
            let target = self.position(invitation.target,game);
            if self.time >= invitation.expires || game.task.is_some() || invitation.order != game.order_serial {
                self.pending = None;
                self.status = "Encuentro cancelado".into();
            } else if game.phase == Phase::Idle && distance(game.ground_position,target) < 1.55 {
                self.start(1,invitation.target,invitation.kind,game);
            }
        }
        if let Some(mut conversation) = self.conversation.take() {
            let has_player = conversation.a == 1 || conversation.b == 1;
            let interrupted = has_player && (!game.talking || game.task.is_some() || game.phase != Phase::Idle);
            if interrupted {
                self.report(&conversation,game,false);
                self.status = "Conversación interrumpida".into();
            } else {
                conversation.elapsed += dt;
                let a = self.position(conversation.a,game);
                let b = self.position(conversation.b,game);
                for (id,from,to) in [(conversation.a,a,b),(conversation.b,b,a)] {
                    let angle = yaw(from,to);
                    if id == 1 {
                        game.frame.rotation[1] = angle;
                        game.frame.arms[1] = -0.25 - (self.time*2.5).sin()*0.13;
                        game.frame.elbows[1] = 0.9;
                        game.frame.head[0] = (self.time*2.0).sin()*0.035;
                    } else if let Some(i) = self.index(id) {
                        let resident = &mut self.residents[i];
                        resident.frame.rotation[1] = angle;
                        resident.frame.gait(self.time,false);
                        resident.frame.arms[0] = -0.22-(self.time*2.2).sin()*0.12;
                        resident.frame.elbows[0] = 0.8;
                        resident.frame.head[0] = (self.time*2.0).sin()*0.035;
                    }
                }
                if conversation.elapsed >= 2.6 && conversation.turn == 0 {
                    conversation.turn = 1;
                    let response = match conversation.kind {
                        SocialKind::Greet => "¡Hola! Me alegra tener compañía.",
                        SocialKind::Chat => if self.time as u64 % 2 == 0 { "Tranquilo. Me está sentando bien estar aquí." }
                                           else { "Con ganas de descansar y charlar un poco." },
                        SocialKind::Joke => "¡Jajaja! A mí me pasa todo el tiempo.",
                    };
                    self.speak(conversation.b,game,response);
                }
                if conversation.elapsed >= 5.2 && conversation.turn == 1 {
                    conversation.turn = 2;
                    self.speak(conversation.a,game,match conversation.kind {
                        SocialKind::Greet => "Luego seguimos hablando. ¡Nos vemos!",
                        SocialKind::Chat => "Podemos repetirlo cuando quieras.",
                        SocialKind::Joke => "¡Al menos no soy el único despistado!",
                    });
                }
                if conversation.elapsed >= 8.0 {
                    self.report(&conversation,game,true);
                    self.status = "El encuentro ha mejorado su relación".into();
                } else { self.conversation = Some(conversation); }
            }
        }
        let busy: Vec<_> = self.conversation.as_ref().map(|c| vec![c.a,c.b]).unwrap_or_default();
        let positions: Vec<_> = self.residents.iter().map(|r| (r.id,r.frame.position)).collect();
        for resident in &mut self.residents {
            if busy.contains(&resident.id) { continue; }
            if self.pending.as_ref().is_some_and(|p| p.target == resident.id) {
                resident.frame.gait(self.time,false); continue;
            }
            resident.wander_in -= dt;
            if let Some(point) = resident.path.front().copied() {
                let current = resident.frame.position;
                let d = distance(current,point);
                if d < 0.015 {
                    resident.frame.position = point;
                    resident.path.pop_front();
                    resident.wait_for = 0.0;
                } else {
                    let fraction = (dt*1.1/d).min(1.0);
                    let next = Vec3::new(current.x+(point.x-current.x)*fraction,0.0,current.z+(point.z-current.z)*fraction);
                    let occupied = distance(next,game.ground_position)<0.6
                        || positions.iter().any(|(id,p)| *id != resident.id && distance(next,*p)<0.58);
                    if !occupied && game.navigation.segment_free(current,next) {
                        resident.frame.position = next;
                        resident.frame.rotation[1] = yaw(current,point);
                        resident.wait_for = 0.0;
                    } else {
                        resident.wait_for += dt;
                        if resident.wait_for > 1.5 { resident.path.clear(); resident.wander_in=0.0; }
                    }
                }
                resident.frame.gait(self.time,!resident.path.is_empty());
                resident.action = "Caminando por casa".into();
            } else {
                resident.frame.gait(self.time,false);
                resident.action = "En casa".into();
            }
            if resident.path.is_empty() && resident.wander_in <= 0.0 {
                let destinations = [Vec3::new(-0.8,0.0,0.0),Vec3::new(0.0,0.0,1.8),
                    Vec3::new(1.2,0.0,-0.8),Vec3::new(-1.4,0.0,2.4),Vec3::new(4.0,0.0,1.8)];
                let point = destinations[(self.wander_sequence+resident.id as usize)%destinations.len()];
                self.wander_sequence += 1;
                if let Some(route) = game.navigation.route(resident.frame.position,point) { resident.path=route; }
                resident.wander_in = 7.0+resident.id as f64;
            }
            // Give way to the player; people never become permanent door blockers.
            if distance(resident.frame.position,game.ground_position)<0.56 && game.phase == Phase::Walking {
                let away = resident.frame.position;
                for side in [-1.0,1.0] {
                    let point = Vec3::new(away.x+side*0.6,0.0,away.z+0.4);
                    if let Some(route) = game.navigation.route(away,point) { resident.path=route; break; }
                }
            }
        }
        self.decision_in -= dt;
        if self.decision_in <= 0.0 && self.conversation.is_none() && self.pending.is_none() {
            self.decision_in = 8.0;
            let mut selected = None;
            for i in 0..self.residents.len() {
                if self.residents[i].cooldown > 0.0 { continue; }
                let from = self.residents[i].frame.position;
                if game.phase == Phase::Idle && !game.talking && distance(from,game.ground_position) < 2.1 {
                    if let Some(kind) = self.residents[i].psychology.social_choice(1) {
                        selected=Some((self.residents[i].id,1,kind)); break;
                    }
                }
                for j in 0..self.residents.len() {
                    if i==j || self.residents[j].cooldown>0.0 { continue; }
                    if distance(from,self.residents[j].frame.position)<2.0 {
                        let target=self.residents[j].id;
                        if let Some(kind)=self.residents[i].psychology.social_choice(target) {
                            selected=Some((self.residents[i].id,target,kind)); break;
                        }
                    }
                }
                if selected.is_some() { break; }
            }
            if let Some((a,b,kind))=selected { self.start(a,b,kind,game); }
        }
    }
}
