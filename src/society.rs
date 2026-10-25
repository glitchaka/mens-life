//! MSPA-backed outdoor residents using the real selectable character catalog.
use crate::{
    character_catalog::{self, CHARACTERS},
    game::{Frame, Game, Navigation, Phase, Vec3, WIDTH},
    psychology::{Psychology, Reaction, Relationship, SocialKind},
    scene::Avatar,
};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Resident {
    pub id: u64,
    pub profile_index: usize,
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
    needs: [f64; 5],
}
#[derive(Clone, Debug)]
struct Invitation { target: u64, kind: SocialKind, expires: f64, order: u64 }
#[derive(Clone, Debug)]
struct Conversation { target: u64, kind: SocialKind, elapsed: f64, turn: u8 }
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
fn distance(a:Vec3,b:Vec3)->f64{(a.x-b.x).hypot(a.z-b.z)}
fn yaw(from:Vec3,target:Vec3)->f64{(target.x-from.x).atan2(target.z-from.z)}
fn house_interior(p:Vec3)->bool{(-4.96..=-1.00).contains(&p.x)&&(-3.48..=3.54).contains(&p.z)}

/// Outdoor NPCs use a second path constraint on top of the player's navigation:
/// the full house footprint is forbidden even though the front door is open to the protagonist.
fn outdoor_route(navigation:&Navigation,from:Vec3,to:Vec3)->Option<VecDeque<Vec3>>{
    let start=navigation.nearest(from)?;let end=navigation.nearest(to)?;
    if house_interior(Navigation::point(end)){return None;}
    let mut prev=vec![None;navigation.free.len()];let mut queue=VecDeque::from([start]);prev[start]=Some(start);
    while prev[end].is_none(){
        let i=queue.pop_front()?;
        for n in [i.checked_sub(1),i.checked_add(1),i.checked_sub(WIDTH),i.checked_add(WIDTH)].into_iter().flatten(){
            if n>=navigation.free.len()||!navigation.free[n]||prev[n].is_some()||(n%WIDTH).abs_diff(i%WIDTH)>1{continue;}
            let a=Navigation::point(i);let b=Navigation::point(n);
            if house_interior(b)||!navigation.segment_free(a,b){continue;}
            prev[n]=Some(i);queue.push_back(n);
        }
    }
    let mut result=VecDeque::new();let mut i=end;
    while i!=start{result.push_front(Navigation::point(i));i=prev[i]?;}
    if distance(from,Navigation::point(start))>0.001{result.push_front(Navigation::point(start));}
    Some(result)
}

impl Society {
    pub fn new(navigation:&Navigation,selected_profile:usize)->Self{
        let mut residents=Vec::new();let mut runtime_id=2u64;
        for(profile_index,spec)in CHARACTERS.iter().enumerate(){if profile_index==selected_profile{continue;}let desired=Vec3::new(spec.outdoor_spawn[0],0.0,spec.outdoor_spawn[2]);let point=navigation.nearest(desired).map(Navigation::point).unwrap_or(desired);let mut avatar=Avatar::default();avatar.name=spec.name.into();avatar.character=profile_index;let psychology=Psychology::new_profile(runtime_id,profile_index as u64+1,0x4d454e530000u64+runtime_id*97+profile_index as u64).expect("MSPA resident profile");residents.push(Resident{id:runtime_id,profile_index,avatar,frame:Frame{position:point,..Frame::default()},reaction:Reaction{remaining:0.0,..Reaction::default()},psychology,relationship:Relationship::default(),action:"Paseando afuera".into(),path:VecDeque::new(),wander_in:1.5+runtime_id as f64*0.7,wait_for:0.0,cooldown:0.0,needs:[85.0,82.0,90.0,75.0,90.0]});runtime_id+=1;}
        let selected=residents.first().map(|r|r.id);Self{residents,selected,status:"Las demás chicas pasean por fuera de la casa".into(),pending:None,conversation:None,time:0.0,decision_in:7.0,wander_sequence:0}
    }
    fn index(&self,id:u64)->Option<usize>{self.residents.iter().position(|r|r.id==id)}
    fn speak(&mut self,id:u64,game:&mut Game,words:&str){if id==1{game.say(words);return;}if let Some(i)=self.index(id){let resident=&mut self.residents[i];resident.reaction=Reaction{expression:resident.psychology.emotion(),words:words.into(),remaining:6.0};}}
    pub fn request(&mut self,target:u64,kind:SocialKind,game:&mut Game){if game.paused||game.creator{return;}let Some(index)=self.index(target)else{return;};self.pending=None;self.conversation=None;self.selected=Some(target);let destination=self.residents[index].frame.position;let mut candidates:Vec<_>=game.navigation.free.iter().enumerate().filter(|(_,free)|**free).map(|(i,_)|Navigation::point(i)).filter(|p|(0.85..=1.35).contains(&distance(*p,destination))).collect();candidates.sort_by(|a,b|distance(*a,game.ground_position).total_cmp(&distance(*b,game.ground_position)));let Some(point)=candidates.into_iter().find(|p|game.navigation.route(game.ground_position,*p).is_some())else{game.say("No encuentro un lugar libre para acercarme.");return;};self.residents[index].path.clear();self.residents[index].wander_in=30.0;game.walk_to(point);self.pending=Some(Invitation{target,kind,expires:self.time+30.0,order:game.order_serial});self.status=format!("Acercándose a {}",self.residents[index].avatar.name);}
    fn start_conversation(&mut self,target:u64,kind:SocialKind,game:&mut Game){let Some(i)=self.index(target)else{return;};game.talking=true;game.action=format!("Conversando con {}",self.residents[i].avatar.name);self.residents[i].path.clear();self.residents[i].action="Conversando".into();self.pending=None;self.conversation=Some(Conversation{target,kind,elapsed:0.0,turn:0});self.status="Conversación en curso".into();let line=character_catalog::dialogue(self.residents[i].profile_index,kind,0);self.speak(target,game,line);}
    fn finish_conversation(&mut self,conversation:&Conversation,game:&mut Game,success:bool){if let Some(i)=self.index(conversation.target){let r=&mut self.residents[i];r.psychology.social_report(1,conversation.kind,success);game.psychology.social_report(r.id,conversation.kind,success);r.relationship=r.psychology.relationship(1);r.cooldown=16.0;r.wander_in=3.0;r.action="Paseando afuera".into();if success{r.needs[3]=(r.needs[3]+8.0).min(100.0);}}game.talking=false;game.decision_in=4.0;if success{game.values[3]=(game.values[3]+8.0).min(100.0);game.say("Me vino bien hablar un rato.");}}
    pub fn tick(&mut self,game:&mut Game,dt:f64){if game.creator||game.paused{return;}let dt=if dt.is_finite(){dt.clamp(0.0,0.05)}else{0.0};self.time+=dt;for resident in &mut self.residents{for need in &mut resident.needs{*need=(*need-dt*0.015).max(0.0);}resident.psychology.advance(dt,resident.needs);resident.reaction.remaining=(resident.reaction.remaining-dt).max(0.0);resident.reaction.expression=resident.psychology.emotion();resident.cooldown=(resident.cooldown-dt).max(0.0);resident.relationship=resident.psychology.relationship(1);}
        if let Some(invitation)=self.pending.clone(){let Some(index)=self.index(invitation.target)else{self.pending=None;return;};let target=self.residents[index].frame.position;if self.time>=invitation.expires||game.task.is_some()||invitation.order!=game.order_serial{self.pending=None;self.status="Encuentro cancelado".into();}else if game.phase==Phase::Idle&&distance(game.ground_position,target)<1.55{self.start_conversation(invitation.target,invitation.kind,game);}}
        if let Some(mut conversation)=self.conversation.take(){if !game.talking||game.task.is_some()||game.phase!=Phase::Idle{self.finish_conversation(&conversation,game,false);self.status="Conversación interrumpida".into();}else{conversation.elapsed+=dt;if let Some(i)=self.index(conversation.target){let p=self.residents[i].frame.position;game.frame.rotation[1]=yaw(game.ground_position,p);self.residents[i].frame.rotation[1]=yaw(p,game.ground_position);if conversation.elapsed>=3.0&&conversation.turn==0{conversation.turn=1;let line=character_catalog::dialogue(self.residents[i].profile_index,conversation.kind,1);self.speak(conversation.target,game,line);}}if conversation.elapsed>=7.0{self.finish_conversation(&conversation,game,true);self.status="La conversación modificó su relación".into();}else{self.conversation=Some(conversation);}}}
        let positions:Vec<_>=self.residents.iter().map(|r|(r.id,r.frame.position)).collect();let outdoor=[Vec3::new(4.8,0.0,4.7),Vec3::new(5.2,0.0,-4.7),Vec3::new(1.7,0.0,-5.2),Vec3::new(-5.1,0.0,-4.7),Vec3::new(-5.3,0.0,4.7),Vec3::new(3.2,0.0,5.4),Vec3::new(5.6,0.0,0.0),Vec3::new(-5.6,0.0,0.0)];
        for resident in &mut self.residents{if self.conversation.as_ref().is_some_and(|c|c.target==resident.id)||self.pending.as_ref().is_some_and(|p|p.target==resident.id){continue;}resident.wander_in-=dt;if let Some(point)=resident.path.front().copied(){let current=resident.frame.position;let d=distance(current,point);if d<0.02{resident.frame.position=point;resident.path.pop_front();resident.wait_for=0.0;}else{let fraction=(dt*1.05/d).min(1.0);let next=Vec3::new(current.x+(point.x-current.x)*fraction,0.0,current.z+(point.z-current.z)*fraction);let occupied=distance(next,game.ground_position)<0.58||positions.iter().any(|(id,p)|*id!=resident.id&&distance(next,*p)<0.55);if !occupied&&!house_interior(next)&&game.navigation.segment_free(current,next){resident.frame.position=next;resident.frame.rotation[1]=yaw(current,point);resident.wait_for=0.0;}else{resident.wait_for+=dt;if resident.wait_for>1.2{resident.path.clear();resident.wander_in=0.0;}}}resident.frame.gait(self.time,!resident.path.is_empty());resident.action="Paseando afuera".into();}else{resident.frame.gait(self.time,false);resident.action="Mirando alrededor".into();}if resident.path.is_empty()&&resident.wander_in<=0.0{for offset in 0..outdoor.len(){let point=outdoor[(self.wander_sequence+offset+resident.profile_index)%outdoor.len()];if let Some(route)=outdoor_route(&game.navigation,resident.frame.position,point){resident.path=route;self.wander_sequence=self.wander_sequence.wrapping_add(1);break;}}resident.wander_in=5.0+resident.id as f64*0.8;}}
        self.decision_in-=dt;if self.decision_in<=0.0&&self.conversation.is_none()&&self.pending.is_none(){self.decision_in=7.0;for resident in &mut self.residents{if resident.cooldown>0.0||game.phase!=Phase::Idle||game.talking{continue;}if distance(resident.frame.position,game.ground_position)<2.0{if let Some(kind)=resident.psychology.social_choice(1){self.pending=Some(Invitation{target:resident.id,kind,expires:self.time+10.0,order:game.order_serial});self.selected=Some(resident.id);break;}}}}
    }
}
