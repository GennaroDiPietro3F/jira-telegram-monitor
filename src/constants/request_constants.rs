
// --- Costanti per i parametri di query ---
pub const JQL_PARAM_KEY: &str = "jql";
pub const JQL_QUERY_VALUE: &str = "project = GLBAM AND status in (Open, Reopened, \"Waiting for customer\", Pending, \"Work in progress\", Resolution, \"Take in Charge\", \"Approved by Customer\", \"Not Certified\", Estimated, Accepted, Planned, Evaluation, \"Customer certification\")";
pub const MAX_RESULTS_PARAM_KEY: &str = "maxResults";
pub const MAX_RESULTS_VALUE: &str = "500";


pub const QUERY_PARAMS_JIRA_ISSUES: [(&str, &str); 2] = [
    (JQL_PARAM_KEY, JQL_QUERY_VALUE),
    (MAX_RESULTS_PARAM_KEY, MAX_RESULTS_VALUE),
];

// --- Costanti per gli header (singole costanti) ---
pub const ACCEPT_HEADER_KEY: &str = "Accept";
pub const ACCEPT_HEADER_VALUE: &str = "application/json";

pub const CONTENT_TYPE_HEADER_KEY: &str = "Content-Type";
pub const CONTENT_TYPE_HEADER_VALUE: &str = "application/json";



// --- Costante per l'URL base dell'API ---
pub const JIRA_GET_ISSUES_OPEN_URL: &str = "https://support.rgigroup.com/rest/api/2/search";



//--- Constanti per richiesta bot Telegram ---
pub const BOT_ID: &str ="XXX";
pub const TELEGRAM_API: &str = "https://api.telegram.org/bot";
pub const CHAT_ID_HEADER : &str = "chat_id";
pub const CHAT_ID_VALUE: &str = "XXX";
pub const TEXT_HEADER: &str = "text";