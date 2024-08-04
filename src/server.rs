use std::path::Path;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use home::home_dir;
use rodio::OutputStream;
use tonic::{transport::Server, Request, Response, Status};

use whisper::whisper_keys_server::{WhisperKeys, WhisperKeysServer};
use whisper::{GetPacksReq, Packs};

use crate::mechvibes::translate_config;
use crate::packs::{self, Pack};
use crate::{keylogger, player, APP_NAME};

use self::whisper::{
    SetPackReq, SetPackRes, SetVolumeReq, SetVolumeRes, TranslateReq, TranslateRes,
};

pub mod whisper {
    #![allow(clippy::pedantic)]

    tonic::include_proto!("whisper");
}

#[derive(Default)]
struct WhisperService {
    pub packs: Arc<Mutex<Vec<String>>>,
    pub current_pack: Arc<Mutex<Option<Pack>>>,
    pub volume: Arc<Mutex<u16>>,
}

#[tonic::async_trait]
impl WhisperKeys for WhisperService {
    async fn get_packs(&self, _: Request<GetPacksReq>) -> Result<Response<Packs>, Status> {
        let Some(home_dir) = home_dir() else {
            return Err(Status::failed_precondition("Couldn't find home directory"));
        };
        let packs_folder = Path::new(&home_dir).join(APP_NAME);
        let packs = packs::list_available(&packs_folder);

        match packs {
            Ok(packs) => {
                self.packs.lock().unwrap().clone_from(&packs);

                let res = Packs { packs };

                Ok(Response::new(res))
            }
            Err(e) => Err(Status::failed_precondition(e.to_string())),
        }
    }

    async fn set_pack(&self, req: Request<SetPackReq>) -> Result<Response<SetPackRes>, Status> {
        let pack_name = req.into_inner().pack;

        if !self.packs.lock().unwrap().contains(&pack_name) {
            return Err(Status::not_found(format!("Pack '{pack_name}' not found")));
        }

        let Some(home_dir) = home::home_dir() else {
            return Err(Status::failed_precondition("Couldn't find home directory"));
        };

        let packs_folder = Path::new(&home_dir).join(APP_NAME);

        let pack = match packs::load_pack(&packs_folder, &pack_name) {
            Ok(pack) => pack,
            Err(e) => return Err(Status::failed_precondition(e.to_string())),
        };

        let response = SetPackRes {
            pack: pack_name,
            volume: u32::from(pack.keys_default_volume),
        };

        *self.volume.lock().unwrap() = pack.keys_default_volume;
        *self.current_pack.lock().unwrap() = Some(pack);

        Ok(Response::new(response))
    }

    async fn set_volume(
        &self,
        req: Request<SetVolumeReq>,
    ) -> Result<Response<SetVolumeRes>, Status> {
        let volume = req.into_inner().volume;
        let response = SetVolumeRes { volume };

        *self.volume.lock().unwrap() = u16::try_from(volume).unwrap_or(u16::MAX);

        Ok(Response::new(response))
    }

    async fn translate(
        &self,
        req: Request<TranslateReq>,
    ) -> Result<Response<TranslateRes>, Status> {
        let path = req.into_inner().path;
        let response = translate_config(&path);

        match response {
            Ok(()) => Ok(Response::new(TranslateRes {})),
            Err(e) => Err(Status::failed_precondition(e.to_string())),
        }
    }
}

pub async fn serve() {
    let addr = "[::1]:50051".parse().unwrap();
    println!("Server started at {addr}");

    let whisper = WhisperService::default();

    let (tx, rx) = mpsc::channel();
    let Ok((_stream, stream_handle)) = OutputStream::try_default() else {
        panic!("Couldn't find an audio output channel");
    };

    keylogger::listen(tx).expect("Couldn't start keylogger");

    let cloned_pack = Arc::clone(&whisper.current_pack);
    let cloned_volume = Arc::clone(&whisper.volume);
    thread::spawn(move || {
        for msg in rx {
            if cfg!(debug_assertions) {
                dbg!(&msg);
            }

            let pack_lock = cloned_pack.lock().unwrap();
            let pack_ref = pack_lock.as_ref();

            if let Some(pack) = pack_ref {
                let buf = pack
                    .keys
                    .get(&msg)
                    .unwrap_or_else(|| match pack.keys.get("unknown") {
                        Some(buf) => buf,
                        None => panic!("Couldn't find 'unknown' key in pack"),
                    });

                if let Err(e) =
                    player::play_sound(&stream_handle, buf, *cloned_volume.lock().unwrap())
                {
                    eprintln!("Error playing sound: {e}");
                }
            }
        }
    });

    Server::builder()
        .add_service(WhisperKeysServer::new(whisper))
        .serve(addr)
        .await
        .unwrap();
}
