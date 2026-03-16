#[derive(Debug,serde::Serialize)]
#[derive(serde::Deserialize)]
#[derive(Clone)] 
pub struct ConvertedIssues {
    pub glbam_ticket: String,
    pub glbam_status: String,
    pub glbam_squad: String,
    pub updated: String,
}
impl ConvertedIssues {
    pub fn new(glbam_ticket: String, glbam_status: String, glbam_squad: String, updated: String ) -> Self {
        ConvertedIssues {
            glbam_ticket,
            glbam_status,
            glbam_squad,
            updated
        }
    }
    
}