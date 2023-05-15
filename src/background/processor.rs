use tokio::sync::broadcast::{Receiver, Sender};

#[derive(Clone,Debug)]
pub enum IPCData {
    SetPlaybackVolume(),
    ForceStopLoop(),
    LoopIndefinitely(),
    LoopXTimes(),
    SeekToPosition(),
    ResumePlayback(),
    PausePlayback(),
    GetMetadata(),
    PlayFromHttp(),
    PlayFromYoutube(),
    JoinChannel(),
    ExitChannel()
}

pub async fn processor(tx: Sender<IPCData>, mut rx: Receiver<IPCData>) {
    while let Ok(msg) = rx.recv().await {
        println!("{:?}",msg);
        match msg {
            IPCData::SetPlaybackVolume() => {}
            IPCData::ForceStopLoop() => {}
            IPCData::LoopIndefinitely() => {}
            IPCData::LoopXTimes() => {}
            IPCData::SeekToPosition() => {}
            IPCData::ResumePlayback() => {}
            IPCData::PausePlayback() => {}
            IPCData::GetMetadata() => {}
            IPCData::PlayFromHttp() => {}
            IPCData::PlayFromYoutube() => {}
            IPCData::JoinChannel() => {}
            IPCData::ExitChannel() => {}
        }
    }
}