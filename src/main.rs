use url::Url;

use oauth2::{
    AuthUrl, Client, ClientCredentialsTokenRequest, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, StandardErrorResponse, TokenResponse, TokenUrl
};
use oauth2::basic::BasicClient;

#[tokio::main]
async fn main() {
    println!("main 処理開始");
    // クライアントIDとシークレットを設定
    let client_id:ClientId = ClientId::new("あなたのクライアントID:".to_string());
    let client_secret:ClientSecret = ClientSecret::new("あなたのクライアントシークレットは ".to_string());

    // 認証とトークン取得のURL
    let auth_url:AuthUrl = AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string()).unwrap();
    let token_url:TokenUrl = TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap();

    // Oauthクライアントを作成
    let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(RedirectUrl::new(url:"http://localhost:8080".to_string()).unwrap());

    // 認証ＵＲＬを生成
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random())
        .add_scope(Scope::new("https://www.googleapis.com/auth/photoslibrary.readonly".to_string()))
        .url();

    println!("以下のURLにアクセスして認証してください:\n{}", auth_url);

    // ユーザーが取得した認証コードを使ってトークンを取得
    // ここでユーザーがブラウザからリダイレクトURLを確認し、認証コードをコピーして入力
}
