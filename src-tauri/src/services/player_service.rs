use crate::repositories::SessionRepository;
use crate::error::AppResult;

pub struct PlayerService;

impl PlayerService {
    pub async fn play_session(
        session_id: i64,
        repository: &dyn SessionRepository,
    ) -> AppResult<()> {
        // Minimal playback stub: load events and log a summary.
        let records = repository.load_events(session_id).await?;

        if records.is_empty() {
            println!("No events in this session.");
            return Ok(());
        }

        println!("Loaded {} events for session {}. Playback is not implemented in this build.", records.len(), session_id);
        Ok(())
    }
}