mod participant;

use anyhow::{anyhow, Result};
use client::{proto, Client, PeerId};
use collections::HashMap;
use gpui::{Entity, ModelContext, ModelHandle, MutableAppContext, Task};
use participant::{LocalParticipant, ParticipantLocation, RemoteParticipant};
use project::Project;
use std::sync::Arc;

pub enum Event {
    PeerChangedActiveProject,
}

pub struct Room {
    id: u64,
    local_participant: LocalParticipant,
    remote_participants: HashMap<PeerId, RemoteParticipant>,
    client: Arc<Client>,
}

impl Entity for Room {
    type Event = Event;
}

impl Room {
    pub fn create(
        client: Arc<Client>,
        cx: &mut MutableAppContext,
    ) -> Task<Result<ModelHandle<Self>>> {
        cx.spawn(|mut cx| async move {
            let room = client.request(proto::CreateRoom {}).await?;
            Ok(cx.add_model(|cx| Self::new(room.id, client, cx)))
        })
    }

    pub fn join(
        id: u64,
        client: Arc<Client>,
        cx: &mut MutableAppContext,
    ) -> Task<Result<ModelHandle<Self>>> {
        cx.spawn(|mut cx| async move {
            let response = client.request(proto::JoinRoom { id }).await?;
            let room_proto = response.room.ok_or_else(|| anyhow!("invalid room"))?;
            let room = cx.add_model(|cx| Self::new(id, client, cx));
            room.update(&mut cx, |room, cx| room.refresh(room_proto, cx))?;
            Ok(room)
        })
    }

    fn new(id: u64, client: Arc<Client>, _: &mut ModelContext<Self>) -> Self {
        Self {
            id,
            local_participant: LocalParticipant {
                projects: Default::default(),
            },
            remote_participants: Default::default(),
            client,
        }
    }

    fn refresh(&mut self, room: proto::Room, cx: &mut ModelContext<Self>) -> Result<()> {
        for participant in room.participants {
            self.remote_participants.insert(
                PeerId(participant.peer_id),
                RemoteParticipant {
                    user_id: participant.user_id,
                    projects: Default::default(), // TODO: populate projects
                    location: ParticipantLocation::from_proto(participant.location)?,
                },
            );
        }
        Ok(())
    }

    pub fn invite(&mut self, user_id: u64, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        todo!()
    }

    pub async fn publish_project(&mut self, project: ModelHandle<Project>) -> Result<()> {
        todo!()
    }

    pub async fn unpublish_project(&mut self, project: ModelHandle<Project>) -> Result<()> {
        todo!()
    }

    pub async fn set_active_project(
        &mut self,
        project: Option<&ModelHandle<Project>>,
    ) -> Result<()> {
        todo!()
    }

    pub async fn mute(&mut self) -> Result<()> {
        todo!()
    }

    pub async fn unmute(&mut self) -> Result<()> {
        todo!()
    }
}
