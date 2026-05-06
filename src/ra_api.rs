use serde::Deserialize;
use reqwest::Client;
use crate::config::Config;
use crate::errors::AppError;

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct UserSummary {
    #[serde(rename = "User")]
    pub user: String,
    
    #[serde(rename = "ULID")]
    pub ulid: Option<String>,
    
    #[serde(rename = "LastActivity")]
    pub last_activity: Option<LastActivity>,
    
    #[serde(rename = "RichPresenceMsg")]
    pub rich_presence_msg: Option<String>,
    
    #[serde(rename = "LastGameID")]
    pub last_game_id: Option<u64>,
    
    #[serde(rename = "LastGame")]
    pub last_game: Option<LastGame>,
    
    #[serde(rename = "RecentlyPlayed")]
    pub recently_played: Vec<RecentlyPlayedGame>,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct LastActivity {
    #[serde(rename = "ID")]
    pub id: u64,
    
    #[serde(rename = "timestamp")]
    pub timestamp: Option<String>,
    
    #[serde(rename = "activitytype")]
    pub activity_type: Option<String>,
    
    #[serde(rename = "data")]
    pub data: Option<String>,
    
    #[serde(rename = "data2")]
    pub data2: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct LastGame {
    #[serde(rename = "ID")]
    pub id: u64,
    
    #[serde(rename = "Title")]
    pub title: String,
    
    #[serde(rename = "ConsoleID")]
    pub console_id: u64,
    
    #[serde(rename = "ConsoleName")]
    pub console_name: String,
    
    #[serde(rename = "ImageIcon")]
    pub image_icon: Option<String>,
    
    #[serde(rename = "ImageTitle")]
    pub image_title: Option<String>,
    
    #[serde(rename = "ImageIngame")]
    pub image_ingame: Option<String>,
    
    #[serde(rename = "ImageBoxArt")]
    pub image_box_art: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct RecentlyPlayedGame {
    #[serde(rename = "GameID")]
    pub game_id: u64,
    
    #[serde(rename = "ConsoleID")]
    pub console_id: u64,
    
    #[serde(rename = "ConsoleName")]
    pub console_name: String,
    
    #[serde(rename = "Title")]
    pub title: String,
    
    #[serde(rename = "ImageIcon")]
    pub image_icon: Option<String>,
    
    #[serde(rename = "ImageTitle")]
    pub image_title: Option<String>,
    
    #[serde(rename = "ImageIngame")]
    pub image_ingame: Option<String>,
    
    #[serde(rename = "ImageBoxArt")]
    pub image_box_art: Option<String>,
    
    #[serde(rename = "LastPlayed")]
    pub last_played: Option<String>,
    
    #[serde(rename = "AchievementsTotal")]
    pub achievements_total: Option<u32>,
}

impl RecentlyPlayedGame {
    pub fn get_game_image_url(&self) -> Option<String> {
        let image_path = self.image_box_art.as_ref()
            .or(self.image_title.as_ref())
            .or(self.image_ingame.as_ref())?;
        
        if image_path.starts_with("http") {
            return Some(image_path.clone());
        }
        
        if image_path.starts_with('/') {
            return Some(format!("https://retroachievements.org{}", image_path));
        }
        
        Some(format!("https://retroachievements.org/Images/{}", image_path))
    }
}

impl UserSummary {
    pub fn is_playing(&self) -> bool {
        self.rich_presence_msg.is_some() && !self.rich_presence_msg.as_ref().unwrap().is_empty()
    }

    pub fn get_current_game(&self) -> Option<&RecentlyPlayedGame> {
        self.recently_played.first()
    }
}

pub struct RaClient {
    client: Client,
    config: Config,
}

impl RaClient {
    pub fn new(config: Config) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, config }
    }

    pub async fn fetch_user_summary(&self) -> Result<UserSummary, AppError> {
        let url = self.config.ra_url();
        
        let response = self.client
            .get(&url)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AppError::RaApi(format!(
                "API returned status: {}",
                response.status()
            )));
        }

        let user_summary: UserSummary = response.json().await?;
        Ok(user_summary)
    }
}