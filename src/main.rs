use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret,
    CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl
};
use oauth2::basic::BasicClient;
use url::Url;

#[tokio::main]
async fn main() {
    println!("main 処理開始");
    // クライアントIDとシークレットを設定
    let client_id = ClientId::new("あなたのクライアントID:".to_string());
    let client_secret = ClientSecret::new("あなたのクライアントシークレットは ".to_string());

    // 認証とトークン取得のURL
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string()).unwrap();
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap();

    // Oauthクライアントを作成
    let client = BasicClient::new(
        client_id, Some(client_secret), auth_url, Some(token_url)
    ).set_redirect_uri(RedirectUrl::new())
}
