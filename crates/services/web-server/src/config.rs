use lib_utils::envs::get_env;
use std::sync::OnceLock;

// 웹 환경변수인 WebConfig를 전역 변수로 반환하는 함수
//
// 실제 사용은 web_config().WEB_FOLDER; 형태로 사용됨.
//
pub fn web_config() -> &'static WebConfig {
	static INSTANCE: OnceLock<WebConfig> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		// WebConfig::load_from_env()로 호출하는 이유는..
		// WebConfig::new() 처럼 연관 함수로 구현되어 있기 때문임.
		// static 변수 이어서가 아님.. 혼란스러워 하지 말 것.
		WebConfig::load_from_env().unwrap_or_else(|ex| {
			panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
		})
	})
}

#[allow(non_snake_case)]
pub struct WebConfig {
	pub WEB_FOLDER: String,		// 웹 폴더 경로
}

impl WebConfig {
	// 실제 환경 변수로 부터 값을 읽는 함수로
	// web_config() 함수를 통해 초기화시 호출 됨.
	fn load_from_env() -> lib_utils::envs::Result<WebConfig> {
		Ok(WebConfig {
			WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
		})
	}
}
